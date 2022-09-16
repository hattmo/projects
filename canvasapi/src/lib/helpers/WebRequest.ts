import https from "https";
import { RequestFunction } from "./ObjectDef";

export default (hostname: string, key: string, httpsOptions?: https.RequestOptions): RequestFunction => {
    return async (
        method: string,
        path: string,
        parameters?: object,
        body?: object,
        headers?: object,
    ) => new Promise<object>((resolve, reject) => {
        let paramString = "";
        if (parameters !== undefined) {
            paramString = Object.keys(parameters).reduce((prev, curr) => `${prev}&${curr}=${parameters[curr]}`, "?");
        }

        const fullheaders = {
            Accept: "application/json+canvas-string-ids",
            Authorization: `Bearer ${key}`,
            ...headers,
        };
        if (body !== undefined) {
            fullheaders["Content-type"] = "application/json";
        }
        const req = https.request(`https://${hostname}${path}${paramString}`, {
            ...httpsOptions,
            headers: fullheaders,
            method,
        }, (response) => {
            if (response.statusCode !== undefined && response.statusCode >= 200 && response.statusCode < 300) {
                let resbody = "";
                response.on("data", (chunk) => {
                    resbody += chunk;
                });
                response.on("end", () => {
                    resolve(JSON.parse(resbody));
                });
                response.on("error", (err) => {
                    reject(err);
                });
            } else {
                reject(response.statusCode);
            }
        });
        req.on("error", (err) => {
            reject(err);
        });
        if (body !== undefined) {
            req.end(JSON.stringify(body));
        } else {
            req.end();
        }
    });
};
