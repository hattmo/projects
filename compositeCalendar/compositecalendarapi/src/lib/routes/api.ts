import { Router } from "express";
import session from "./api/session";
import settings from "./api/settings";
import calendars from "./api/calendars";
import { IDriveModel } from "../models/drive";
import { IDatabaseModel } from "../models/db";
import { ICalendarModel } from "../models/calendars";

export default (drive: IDriveModel, database: IDatabaseModel, calendarModel: ICalendarModel) => {
    const router = Router();
    router.use("/session", session(database));
    router.use("/settings", settings(database, drive));
    router.use("/calendars", calendars(database, calendarModel));
    return router;
};
