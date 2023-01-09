import { SocketEvent } from './socket-events';
import { User } from './user.model';

export interface Comment extends SocketEvent {
   user: User;
   body: string;
}
