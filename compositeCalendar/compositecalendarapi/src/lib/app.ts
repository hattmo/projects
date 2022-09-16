import express from "express";
import cookieParser from "cookie-parser";
import api from "./routes/api";
import setUserID from "./middleware/setUserID";
import { IDatabaseModel } from "./models/db";
import { IDriveModel } from "./models/drive";
import { ICalendarModel } from "./models/calendars";

export default (drive: IDriveModel, database: IDatabaseModel, calendarModel: ICalendarModel) => {

  const app = express();
  app.use(cookieParser());
  app.use(setUserID(database));
  app.use("/api", api(drive, database, calendarModel));

  app.get("/", (_req, res) => {
    res.sendStatus(200);
  })

  app.use((_req, _res, next) => {
    next(404);
  });

  app.use((err, _req, res, _next) => {

    if (err === 404) {
      res.sendStatus(404);
    } else {
      res.sendStatus(500);
      process.stderr.write(err + "\n");
    }
  });

  return app;

}