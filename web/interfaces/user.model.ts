export interface User {
   id: string;
   displayName: string;
   displayColour: string;
   createdAt: Date;
   previousNames: string[];
   nameChangedAt: Date;
   scopes: string[];
   authenticated: boolean;
}
