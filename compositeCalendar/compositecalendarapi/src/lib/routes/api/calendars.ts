import { Router } from "express";
import { IDatabaseModel } from "../../models/db";
import { ICalendarModel } from "../../models/calendars";

export default (database: IDatabaseModel, calendarModel: ICalendarModel) => {
    const router = Router();
    router.get("/", (req, res, next) => {
        if (!req.email) {
            next("No valid session");
            return;
        }
        database.getOauthClient(req.email)
            .then(auth => calendarModel.getCalendarList(auth))
            .then(callist => res.json({ calendars: callist }))
            .catch(next);
    });
    return router;
}