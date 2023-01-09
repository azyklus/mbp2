import { FC, useEffect, useState } from 'react';
import { atom, selector, useRecoilState, useSetRecoilState, RecoilEnv } from 'recoil';
import { useMachine } from '@xstate/react';
import { MakeEmptyClientConfig, ClientConfig } from '../../interfaces/client-config.model';
import ClientConfigService from '../../services/client-config-service';
import ChatService from '../../services/chat-service';
import WebsocketService from '../../services/websocket-service';
import { ChatMessage } from '../../interfaces/chat-message.model';
import { CurrentUser } from '../../interfaces/current-user';
import { ServerStatus, MakeEmptyServerStatus } from '../../interfaces/server-status.model';
import AppStateModel, {
   AppStateEvent,
   AppStateOptions,
   MakeEmptyAppState,
} from './application-state';
import { setLocalStorage, getLocalStorage } from '../../utils/localStorage';
import {
   ConnectedClientInfoEvent,
   MessageType,
   ChatEvent,
   MessageVisibilityEvent,
   SocketEvent,
} from '../../interfaces/socket-events';
import { MergeMeta } from '../../utils/helpers';
import handleConnectedClientInfoMessage from './eventhandlers/connected-client-info-handler';
import ServerStatusService from '../../services/status-service';
import handleNameChangeEvent from './eventhandlers/handleNameChangeEvent';
import { DisplayableError } from '../../types/displayable-error';

RecoilEnv.RECOIL_DUPLICATE_ATOM_KEY_CHECKING_ENABLED = false;

const SERVER_STATUS_POLL_DURATION = 5000;
const ACCESS_TOKEN_KEY = 'accessToken';

let serverStatusRefreshPoll: ReturnType<typeof setInterval>;

// Server status is what gets updated such as viewer count, durations,
// stream title, online/offline state, etc.
export const serverStatusState = atom<ServerStatus>({
   key: 'serverStatusState',
   default: MakeEmptyServerStatus(),
});

// The config that comes from the API.
export const ClientConfigStateAtom = atom({
   key: 'clientConfigState',
   default: MakeEmptyClientConfig(),
});

export const AccessTokenAtom = atom<string>({
   key: 'accessTokenAtom',
   default: null,
});

export const CurrentUserAtom = atom<CurrentUser>({
   key: 'currentUserAtom',
   default: null,
});

export const ChatMessagesAtom = atom<ChatMessage[]>({
   key: 'chatMessages',
   default: [] as ChatMessage[],
});

export const ChatAuthenticatedAtom = atom<boolean>({
   key: 'chatAuthenticatedAtom',
   default: false,
});

export const WebsocketServiceAtom = atom<WebsocketService>({
   key: 'websocketServiceAtom',
   default: null,
   dangerouslyAllowMutability: true,
});

export const AppStateAtom = atom<AppStateOptions>({
   key: 'appState',
   default: MakeEmptyAppState(),
});

export const IsMobileAtom = atom<boolean | undefined>({
   key: 'isMobileAtom',
   default: undefined,
});

export const ChatVisibleToggleAtom = atom<boolean>({
   key: 'chatVisibilityToggleAtom',
   default: true,
});

export const IsVideoPlayingAtom = atom<boolean>({
   key: 'isVideoPlayingAtom',
   default: false,
});

export const FatalErrorStateAtom = atom<DisplayableError>({
   key: 'fatalErrorStateAtom',
   default: null,
});

export const ClockSkewAtom = atom<Number>({
   key: 'clockSkewAtom',
   default: 0.0,
});

export const RemovedMessageIdsAtom = atom<string[]>({
   key: 'removedMessageIds',
   default: [],
});

// Chat is visible if the user wishes it to be visible AND the required
// chat state is set.
export const IsChatVisibleSelector = selector({
   key: 'isChatVisibleSelector',
   get: ({ get }) => {
      const state: AppStateOptions = get(AppStateAtom);
      const userVisibleToggle: boolean = get(ChatVisibleToggleAtom);
      const accessToken: string = get(AccessTokenAtom);
      return accessToken && state.chatAvailable && userVisibleToggle;
   },
});

export const IsChatAvailableSelector = selector({
   key: 'isChatAvailableSelector',
   get: ({ get }) => {
      const state: AppStateOptions = get(AppStateAtom);
      const accessToken: string = get(AccessTokenAtom);
      return accessToken && state.chatAvailable;
   },
});

// We display in an "online/live" state as long as video is actively playing.
// Even during the time where technically the server has said it's no longer
// live, however the last few seconds of video playback is still taking place.
export const IsOnlineSelector = selector({
   key: 'isOnlineSelector',
   get: ({ get }) => {
      const state: AppStateOptions = get(AppStateAtom);
      const isVideoPlaying: boolean = get(IsVideoPlayingAtom);
      return state.videoAvailable || isVideoPlaying;
   },
});

export const VisibleChatMessagesSelector = selector<ChatMessage[]>({
   key: 'visibleChatMessagesSelector',
   get: ({ get }) => {
      const messages: ChatMessage[] = get(ChatMessagesAtom);
      const removedIds: string[] = get(RemovedMessageIdsAtom);
      return messages.filter(message => !removedIds.includes(message.id));
   },
});

export const ClientConfigStore: FC = () => {
   const [appState, appStateSend, appStateService] = useMachine(AppStateModel);
   const [currentUser, setCurrentUser] = useRecoilState(CurrentUserAtom);
   const setChatAuthenticated = useSetRecoilState<boolean>(ChatAuthenticatedAtom);
   const [clientConfig, setClientConfig] = useRecoilState<ClientConfig>(ClientConfigStateAtom);
   const [serverStatus, setServerStatus] = useRecoilState<ServerStatus>(ServerStatusState);
   const setClockSkew = useSetRecoilState<Number>(ClockSkewAtom);
   const [chatMessages, setChatMessages] = useRecoilState<ChatMessage[]>(ChatMessagesAtom);
   const [accessToken, setAccessToken] = useRecoilState<string>(AccessTokenAtom);
   const setAppState = useSetRecoilState<AppStateOptions>(AppStateAtom);
   const setGlobalFatalErrorMessage = useSetRecoilState<DisplayableError>(FatalErrorStateAtom);
   const setWebsocketService = useSetRecoilState<WebsocketService>(WebsocketServiceAtom);
   const [hiddenMessageIds, setHiddenMessageIds] = useRecoilState<string[]>(RemovedMessageIdsAtom);
   const [, setHasLoadedStatus] = useState(false);
   const [hasLoadedConfig, setHasLoadedConfig] = useState(false);

   let ws: WebsocketService;

   const setGlobalFatalError = (title: string, message: string) => {
      setGlobalFatalErrorMessage({
         title,
         message,
      });
   };
   const sendEvent = (event: string) => {
      // console.debug('---- sending event:', event);
      appStateSend({ type: event });
   };

   const handleStatusChange = (status: ServerStatus) => {
      if (appState.matches('loading')) {
         sendEvent(AppStateEvent.Loaded);
         return;
      }

      if (status.online && appState.matches('ready')) {
         sendEvent(AppStateEvent.Online);
      } else if (!status.online && !appState.matches('ready.offline')) {
         sendEvent(AppStateEvent.Offline);
      }
   };

   const updateClientConfig = async () => {
      try {
         const config = await ClientConfigService.getConfig();
         setClientConfig(config);
         setGlobalFatalErrorMessage(null);
         setHasLoadedConfig(true);
      } catch (error) {
         setGlobalFatalError(
            'Unable to reach Owncast server',
            `Owncast cannot launch. Please make sure the Owncast server is running.`,
         );
         console.error(`ClientConfigService -> getConfig() ERROR: \n${error}`);
      }
   };

   const updateServerStatus = async () => {
      try {
         const status = await ServerStatusService.getStatus();
         setServerStatus(status);
         setHasLoadedStatus(true);
         const { serverTime } = status;

         const clockSkew = new Date(serverTime).getTime() - Date.now();
         setClockSkew(clockSkew);

         setGlobalFatalErrorMessage(null);
      } catch (error) {
         sendEvent(AppStateEvent.Fail);
         setGlobalFatalError(
            'Unable to reach Owncast server',
            `Owncast cannot launch. Please make sure the Owncast server is running.`,
         );
         console.error(`serverStatusState -> getStatus() ERROR: \n${error}`);
      }
   };

   const handleUserRegistration = async (optionalDisplayName?: string) => {
      const savedAccessToken = getLocalStorage(ACCESS_TOKEN_KEY);
      if (savedAccessToken) {
         setAccessToken(savedAccessToken);
         return;
      }

      try {
         sendEvent(AppStateEvent.NeedsRegister);
         const response = await ChatService.registerUser(optionalDisplayName);
         const { accessToken: newAccessToken, displayName: newDisplayName, displayColor } = response;
         if (!newAccessToken) {
            return;
         }

         setCurrentUser({
            ...currentUser,
            displayName: newDisplayName,
            displayColour,
         });
         setAccessToken(newAccessToken);
         setLocalStorage(ACCESS_TOKEN_KEY, newAccessToken);
      } catch (e) {
         sendEvent(AppStateEvent.Fail);
         console.error(`ChatService -> registerUser() ERROR: \n${e}`);
      }
   };

   const resetAndReAuth = () => {
      setLocalStorage(ACCESS_TOKEN_KEY, '');
      setAccessToken(null);
      handleUserRegistration();
   };

   const handleMessageVisibilityChange = (message: MessageVisibilityEvent) => {
      const { ids, visible } = message;
      if (visible) {
         const updatedIds = hiddenMessageIds.filter(id => !ids.includes(id));
         setHiddenMessageIds(updatedIds);
      } else {
         const updatedIds = [...hiddenMessageIds, ...ids];
         setHiddenMessageIds(updatedIds);
      }
   };

   const handleMessage = (message: SocketEvent) => {
      switch (message.type) {
         case MessageType.ERROR_NEEDS_REGISTRATION:
            resetAndReAuth();
            break;
         case MessageType.CONNECTED_USER_INFO:
            handleConnectedClientInfoMessage(
               message as ConnectedClientInfoEvent,
               setChatAuthenticated,
               setCurrentUser,
            );
            setChatMessages(currentState => [...currentState, message as ChatEvent]);
            break;
         case MessageType.CHAT:
            setChatMessages(currentState => [...currentState, message as ChatEvent]);
            break;
         case MessageType.NAME_CHANGE:
            handleNameChangeEvent(message as ChatEvent, setChatMessages);
            break;
         case MessageType.USER_JOINED:
            setChatMessages(currentState => [...currentState, message as ChatEvent]);
            break;
         case MessageType.SYSTEM:
            setChatMessages(currentState => [...currentState, message as ChatEvent]);
            break;
         case MessageType.CHAT_ACTION:
            setChatMessages(currentState => [...currentState, message as ChatEvent]);
            break;
         case MessageType.VISIBILITY_UPDATE:
            handleMessageVisibilityChange(message as MessageVisibilityEvent);
            break;
         default:
            console.error('Unknown socket message type: ', message.type);
      }
   };

   const getChatHistory = async () => {
      try {
         const messages = await ChatService.getChatHistory(accessToken);
         setChatMessages(currentState => [...currentState, ...messages]);
      } catch (error) {
         console.error(`ChatService -> getChatHistory() ERROR: \n${error}`);
      }
   };

   const startChat = async () => {
      try {
         const { socketHostOverride } = clientConfig;
         const host = socketHostOverride || window.location.toString();
         ws = new WebsocketService(accessToken, '/ws', host);
         ws.handleMessage = handleMessage;
         setWebsocketService(ws);
      } catch (error) {
         console.error(`ChatService -> startChat() ERROR: \n${error}`);
      }
   };

   const handleChatNotification = () => {
   };

   // Read the config and status on initial load from a JSON string that lives
   // in window. This is placed there server-side and allows for fast initial
   // load times because we don't have to wait for the API calls to complete.
   useEffect(() => {
      try {
         if ((window as any).configHydration) {
            const config = JSON.parse((window as any).configHydration);
            setClientConfig(config);
            setHasLoadedConfig(true);
         }
      } catch (e) {
         console.error('Error parsing config hydration', e);
      }

      try {
         if ((window as any).statusHydration) {
            const status = JSON.parse((window as any).statusHydration);
            setServerStatus(status);
            setHasLoadedStatus(true);
         }
      } catch (e) {
         console.error('error parsing status hydration', e);
      }
   }, []);

   useEffect(() => {
      handleStatusChange(serverStatus);
   }, [serverStatus]);

   useEffect(() => {
      if (!clientConfig.chatDisabled && accessToken && hasLoadedConfig) {
         startChat();
      }
   }, [hasLoadedConfig, accessToken]);

   // Notify about chat activity when backgrounded.
   useEffect(() => {
      handleChatNotification();
   }, [chatMessages]);

   useEffect(() => {
      updateClientConfig();
      handleUserRegistration();
      updateServerStatus();

      clearInterval(serverStatusRefreshPoll);
      serverStatusRefreshPoll = setInterval(() => {
         updateServerStatus();
      }, SERVER_STATUS_POLL_DURATION);

      return () => {
         clearInterval(serverStatusRefreshPoll);
      };
   }, []);

   useEffect(() => {
      if (!accessToken) {
         return;
      }

      getChatHistory();
   }, [accessToken]);

   useEffect(() => {
      appStateService.onTransition(state => {
         const metadata = MergeMeta(state.meta) as AppStateOptions;

         // console.debug('--- APP STATE: ', state.value);
         // console.debug('--- APP META: ', metadata);

         setAppState(metadata);
      });
   }, []);

   return null;
};