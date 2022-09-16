import { google } from "googleapis";
import { ISettingCalendar } from "../../types";

export interface ICalendarModel {
    getCalendarList: (auth) => Promise<ISettingCalendar[]>;
}

export default (): ICalendarModel => {
    return {
        getCalendarList: async (auth) => {
            const calendar = google.calendar({ version: "v3", auth })
            const calRes = await calendar.calendarList.list();
            if (!calRes.data.items) {
                throw new Error();
            }
            return calRes.data.items
                .filter((item): item is ISettingCalendar => {
                    return (
                        item.id !== undefined
                        && item.summary !== undefined
                        && item.accessRole !== undefined
                        && item.id !== null
                        && item.summary !== null
                        && item.accessRole !== null
                    );
                })
                .map(({ id, summary, accessRole }) => ({ id, summary, accessRole }));
        }
    };
}
