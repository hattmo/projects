import { Router } from "express";
import { IDriveModel } from "../../models/drive";
import { IDatabaseModel } from "../../models/db";
import express from "express";
export default (database: IDatabaseModel, drive: IDriveModel) => {
    const router = Router();
    router.get("/", (req, res, next) => {
        if (!req.email) {
            next("No valid session");
            return;
        }
        database.getOauthClient(req.email)
            .then(auth => drive.getSettings(auth))
            .then(settings => res.json({ settings }))
            .catch(next);
    });
    router.post("/", express.json(), (req, res, next) => {
        if (!req.email || !req.body) {
            next("No valid session");
            return;
        }
        database.getOauthClient(req.email)
            .then(auth => drive.setSettings(auth, req.body))
            .then(() => res.json({ settingsUpdated: true }))
            .catch(next);
    });
    return router;
}