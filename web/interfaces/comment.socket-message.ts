import { SocketEvent } from './socket-events';

export interface CommentMessage extends SocketEvent {
   title: string;
   body: string;
   image: string;
   link: string;
}
