#!/usr/bin/env node
import { promises as fsp } from "fs";
import { loadAssets } from "../lib/assetLoader";
import { IAsset } from "../lib/interfaces/assetInf";
import { createAssetDirectory } from "../lib/models/fileIO";

const WORKINGDIR = process.cwd();
const ASSETDIR = `${WORKINGDIR}/assets`;
(async () => {
    try {
        const data = await fsp.readFile(`${WORKINGDIR}/package.json`);
        const conf = JSON.parse(data.toString("utf-8"));
        if (conf.assets && Array.isArray(conf.assets)) {
            const assets = (conf.assets as IAsset[]);
            await createAssetDirectory(ASSETDIR);
            await loadAssets(ASSETDIR, assets);
        } else {
            process.stderr.write("Package.json does not have an asset property");
        }
    } catch (error) {
        process.stderr.write(error);
    }
})();
