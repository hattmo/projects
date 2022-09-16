import { ISetting } from "../../types";
import { google } from "googleapis";

export interface IDriveModel {
    getSettings: (auth) => Promise<ISetting[]>;
    setSettings: (auth, payload) => Promise<void>;
}

export default (): IDriveModel => {
    return {
        getSettings: async (auth) => {
            const drive = google.drive({ version: "v3", auth });
            const listRes = await drive.files.list({ spaces: "appDataFolder" });
            const dataFileid = listRes.data.files?.find(item => item.name === "compositeCalendarData")?.id;
            if (!dataFileid) {
                throw new Error("Cannot find file compositeCalendarData\n");
            }
            const constentRes = await drive.files.get({ fileId: dataFileid, alt: "media" }, { responseType: "text" });
            // TODO: UNSAFE TYPE CHECK MAGIC
            return JSON.parse(constentRes.data as string) as ISetting[];
        },




        setSettings: async (auth, payload) => {
            const drive = google.drive({ version: "v3", auth });
            const listRes = await drive.files.list({ spaces: "appDataFolder" });
            const dataFileid = listRes.data.files?.find(item => item.name === "compositeCalendarData")?.id;
            if (!dataFileid) {
                await drive.files.create({
                    requestBody: { name: "compositeCalendarData", parents: ["appDataFolder"] },
                    media: { body: payload, mimeType: "application/json" }
                });
            } else {
                await drive.files.update({
                    fileId: dataFileid,
                    requestBody: { name: "compositeCalendarData", parents: ["appDataFolder"] },
                    media: { body: payload, mimeType: "application/json" }
                })
            }
        }
    }
}
