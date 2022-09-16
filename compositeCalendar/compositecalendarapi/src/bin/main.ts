#!/usr/bin/env node

import http from "http";
import app from "../lib/app";
import { AddressInfo } from "net";
import db from "../lib/models/db";
import drive from "../lib/models/drive";
import calendarModel from "../lib/models/calendars";
const {
    OAUTH_CLIENT_ID,
    OAUTH_CLIENT_SECRET,
    DB_CONNECTION,
    DB_USERNAME,
    DB_PASSWORD,
    NODEPORT,
} = process.env;

if (!(
    OAUTH_CLIENT_ID &&
    OAUTH_CLIENT_SECRET &&
    DB_CONNECTION
)) {
    process.stderr.write("Environment variables not set\n");
    process.exit(1);
}

const port = NODEPORT ?? 80;

db(DB_CONNECTION, DB_USERNAME, DB_PASSWORD).then((database) => {

    const appInstance = app(drive(), database, calendarModel());
    appInstance.set("port", port);
    appInstance.set("trust proxy", true);
    const server = http.createServer(appInstance);

    const onError = (error) => {
        if (error.syscall !== "listen") {
            throw error;
        }

        switch (error.code) {
            case "EACCES":
                process.stderr.write(`Port ${port} requires elevated privileges\n`);
                process.exit(1);
                break;
            case "EADDRINUSE":
                process.stderr.write(`Port ${port} is already in use\n`);
                process.exit(1);
                break;
            default:
                throw error;
        }
    };

    const onListening = () => {
        const addr = server.address() as AddressInfo;
        process.stdout.write(`Listening on port ${addr.port}\n`);
    };
    server.on("error", onError);
    server.on("listening", onListening);
    server.listen(port);

}).catch((_e) => {
    process.stderr.write("Failed to connect to Database, exiting...");
    process.exit(1);
});
