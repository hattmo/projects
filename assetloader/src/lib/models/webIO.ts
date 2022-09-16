import https from "https";
import stream from "stream";

export default async function getReadStream(url: string): Promise<stream.Readable> {
    return new Promise((resolve, reject) => {
        const req = https.get(url, (res) => {
            resolve(res);
        });
        req.on("error", (err) => {
            reject(err);
        });
    });
}
