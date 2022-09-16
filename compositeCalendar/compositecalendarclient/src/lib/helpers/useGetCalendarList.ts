import { useEffect } from "react";
import { ISettingCalendar } from "../../types";

type callback = (cal: ISettingCalendar[]) => void;

export default (cb: callback) => {
    useEffect(() => {
        fetch("/api/calendars")
            .then((res) => {
                if (res.ok) {
                    return res.json();
                }
                throw new Error("Error requesting calendars..");
            })
            .then((res) => {
                return res.calendars.map((item) => {
                    return {
                        id: item.id,
                        name: item.summary,
                        accessRole: item.accessRole,
                    };
                });
            })
            .then(cb)
            .catch(() => { cb([]) })
    }, []);
};
