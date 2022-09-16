import { RequestHandler } from "express";
import { IDatabaseModel } from "../models/db";


export default (database: IDatabaseModel): RequestHandler => {
    return (req, res, next) => {
        if (req.cookies.ccsession !== undefined) {
            database.getEmail(req.cookies.ccsession, req.ip)
                .then((email) => {
                    req.email = email;
                    next();
                })
                .catch(() => {
                    console.log("cleared")
                    res.clearCookie("ccsession");
                    next();
                })
        } else {
            next();
        }
    };
}