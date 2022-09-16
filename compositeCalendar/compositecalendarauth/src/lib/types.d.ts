import { Credentials } from "google-auth-library";

interface IAccountDocument {
    email: string;
    credentials: Credentials;
    lastUpdate: number;
    session: Session[];
}
interface Session {
    cookie: string;
    src: string;
    lastLogin: number;
}
