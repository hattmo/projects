import { Router } from "express";
import { google } from "googleapis";


export default (clientId: string, scope: string, redirectUri: string) => {
    const oauth2Client = new google.auth.OAuth2();
    const forwardUrl = oauth2Client.generateAuthUrl({
        access_type: "offline",
        prompt: "consent",
        response_type: "code",
        client_id: clientId,
        redirect_uri: redirectUri,
        scope,
    })
    const router = Router();

    router.get("/", (_req, res, _next) => {
        res.redirect(forwardUrl);
    })
    return router;
};
