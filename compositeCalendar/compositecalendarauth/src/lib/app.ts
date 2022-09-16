import express from "express";
import login from "./routes/login";
import auth from "./routes/auth";
import { IDatabaseModel } from "./db";

export default (
  clientId: string,
  clientSecret: string,
  authRedirect: string,
  successRedirect: string,
  failureRedirect: string,
  scopes: string,
  database: IDatabaseModel
) => {

  const app = express();

  app.use("/auth", auth(
    clientId,
    clientSecret,
    authRedirect,
    successRedirect,
    failureRedirect,
    database,
  ));

  app.use("/login", login(
    clientId,
    scopes,
    authRedirect,
  ));

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