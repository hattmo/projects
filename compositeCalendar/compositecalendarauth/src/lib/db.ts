import { MongoClientOptions, MongoClient } from "mongodb";
import { Credentials } from "google-auth-library";
import { IAccountDocument } from "./types";
export interface IDatabaseModel {
    updateUser: (tokens: Credentials, email: string, cookie: string, src: string) => Promise<void>;
}

export default async (dbconnection: string, dbusername?: string, dbpassword?: string): Promise<IDatabaseModel> => {
    const connectionSetting: MongoClientOptions = {};
    if (dbusername !== undefined && dbpassword !== undefined) {
        connectionSetting.auth = { user: dbusername, password: dbpassword };
    }
    const client = new MongoClient(dbconnection, connectionSetting);
    const db = (await client.connect()).db("compositecalendar");

    return {
        updateUser: async (tokens: Credentials, email: string, cookie: string, src: string) => {
            const { access_token, expiry_date, refresh_token } = tokens;
            const currentTime = new Date().getTime();
            await db.collection<IAccountDocument>("accounts").updateOne(
                { email },
                {
                    $setOnInsert: { email },
                    $set: {
                        credentials: { access_token, expiry_date, refresh_token, },
                        lastUpdate: currentTime,
                    },
                    $push: {
                        session: { cookie, src, lastLogin: currentTime }
                    }
                },
                { upsert: true },
            );
        },
    };
};
