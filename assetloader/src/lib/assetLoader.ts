import { IAsset } from "./interfaces/assetInf";
import getWriteStream from "./models/fileIO";
import getReadStream from "./models/webIO";

export async function loadAssets(assetDir: string, assets: IAsset[]) {
    (await Promise.all(assets.map(async (asset) => {
        return {
            read: await getReadStream(asset.uri),
            write: getWriteStream(`${assetDir}/${asset.filename}`),
        };
    }))).forEach((assetStreams) => {
        assetStreams.read.pipe(assetStreams.write);
        assetStreams.read.on("error", (err) => {
            throw err;
        });
    });
}
