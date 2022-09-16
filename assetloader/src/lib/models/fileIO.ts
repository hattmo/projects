import { promises as fsp } from "fs";
import fs from "fs";
import stream from "stream";

export async function createAssetDirectory(dirname: string): Promise<void> {
    try {
        await fsp.access(dirname, fs.constants.W_OK);
        await Promise.all((await fsp.readdir(dirname))
            .map(async (file) => fsp.unlink(`${dirname}/${file}`)));
    } catch (error) {
        await fsp.mkdir(dirname);
    }
}

export default function getWriteStream(filename: string): stream.Writable {
    return fs.createWriteStream(filename, { encoding: "binary" });
}
