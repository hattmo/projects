import { MongoClientOptions, MongoClient } from "mongodb";
import { IAccountDocument } from "../../types";
import { google } from "googleapis";

export interface IDatabaseModel {
    getEmail: (cookie: string, src: string) => Promise<string>;
    getOauthClient: (email: string) => Promise<any>;
    clearSession: (cookie: string, src: string) => Promise<void>;
}

export default async (dbconnection: string, dbusername?: string, dbpassword?: string): Promise<IDatabaseModel> => {
    const connectionSetting: MongoClientOptions = {};
    if (dbusername !== undefined && dbpassword !== undefined) {
        connectionSetting.auth = { user: dbusername, password: dbpassword };
    }
    const client = new MongoClient(dbconnection, connectionSetting);
    const db = (await client.connect()).db("compositecalendar");
    return {
        getEmail: async (cookie: string, src: string): Promise<string> => {
            const currentTime = new Date().getTime();
            const response = (await db.collection<IAccountDocument>("accounts").findOneAndUpdate(
                { session: { $elemMatch: { cookie, src } } },
                { $set: { "session.$[thisSession].lastLogin": currentTime } },
                { arrayFilters: [{ "thisSession.cookie": { $eq: cookie }, "thisSession.src": { $eq: src } }] })).value;
            if (response) {
                return response.email;
            } else {
                throw new Error("No Session found");
            }
        },
        getOauthClient: async (email: string) => {
            const response = await db.collection<IAccountDocument>("accounts").findOne({ email });
            if (response?.credentials) {
                const oauth = new google.auth.OAuth2();
                oauth.setCredentials(response.credentials);
                return oauth;
            } else {
                throw new Error("No Session found");
            }
        },
        clearSession: async (cookie: string, src: string) => {
            if ((await db.collection<IAccountDocument>("sessions").updateOne(
                { session: { $elemMatch: { cookie, src } } },
                { $pull: { session: { cookie, src } } })).modifiedCount !== 1) {
                throw new Error("Failed to logout properly");
            };
        }
    };
};
