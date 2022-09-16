import { Router } from "express";
import { v4 as uuid } from "uuid";
import { IDatabaseModel } from "../db";
import { google } from "googleapis";


export default (
    clientId: string,
    clientSecret: string,
    authRedirect: string,
    successRedirect: string,
    failureRedirect: string,
    db: IDatabaseModel,
) => {
    const router = Router();
    router.get("/", async (req, res, _next) => {
        if (!(req.query.code !== undefined && typeof (req.query.code) === "string")) {
            res.redirect(failureRedirect);
            return;
        }
        const cookie = uuid();
        const oauth2Client = new google.auth.OAuth2({ clientId, clientSecret, redirectUri: authRedirect });
        oauth2Client.getToken(req.query.code).then(creds => {
            oauth2Client.setCredentials(creds.tokens);
            return google.oauth2({ version: "v2", auth: oauth2Client }).userinfo.v2.me.get().then(userinfo => {
                if (!userinfo.data.email) {
                    throw new Error("no email set");
                }
                return db.updateUser(creds.tokens, userinfo.data.email, cookie, req.ip);
            })
        }).then(() => {
            res.cookie("ccsession", cookie, { httpOnly: true, maxAge: 7776000000 });
            res.redirect(successRedirect);
        }).catch((_error) => {
            res.redirect(failureRedirect);
        });
    })
    return router;
};