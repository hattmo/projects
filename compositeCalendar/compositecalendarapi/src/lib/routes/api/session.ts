import { Router } from "express";
import { IDatabaseModel } from "../../models/db";

export default (database: IDatabaseModel) => {
    const router = Router();
    router.get("/", (req, res) => {
        res.json({ loggedin: (req.email !== undefined) })
    });
    router.delete("/", (req, res, next) => {
        const ccsession = req.cookies.ccsession;
        if (req.email && typeof ccsession === "string") {
            database.clearSession(ccsession, req.ip).then(() => {
                res.clearCookie("ccsession");
                console.log("logged out")
                res.json({ loggedin: false });
            }).catch(next);
        } else {
            next("No valid session");
        }
    })
    return router;
}