import { app, BrowserWindow, ipcMain } from "electron";
import { join as p } from "path";
import csv from "csv-parser";
import pdftk from "node-pdftk";
import { promises as fs } from "fs";
import { homedir } from "os";
import { v4 as genID } from "uuid";

if (require("electron-squirrel-startup")) {
  // eslint-disable-line global-require
  app.quit();
}

const persistentDirPath = p(homedir(), ".pdfgen");
const templateDirPath = p(persistentDirPath, "templates");
const dataDirPath = p(persistentDirPath, "data");
const projectsFilePath = p(persistentDirPath, "projects");

const defaultProject: PersistentData = {
  globalData: {},
  headers: ["name", "age", "phone"],
  data: [],
  template: "",
};

const firstProjectFile = (dataID: string): ProjectsFile => {
  return {
    current: "default",
    projects: { default: dataID },
  };
};

const getProjectsFileData = async (): Promise<ProjectsFile> => {
  return JSON.parse(
    (await fs.readFile(projectsFilePath)).toString("utf-8")
  ) as ProjectsFile;
};

const checkDataDir = async (): Promise<void> => {
  try {
    await fs.stat(persistentDirPath);
  } catch (e) {
    const dataID = genID();
    await fs.mkdir(persistentDirPath);
    await Promise.all([
      fs.writeFile(
        projectsFilePath,
        JSON.stringify(firstProjectFile(dataID)),
        "utf-8"
      ),
      fs.mkdir(templateDirPath),
      fs.mkdir(dataDirPath),
    ]);
    await fs.writeFile(p(dataDirPath, dataID), JSON.stringify(defaultProject));
  }
};

ipcMain.handle("getLastProject", async () => {
  return (await getProjectsFileData()).current;
});

ipcMain.handle("getProjects", async () => {
  const projFile = await getProjectsFileData();
  return Object.keys(projFile.projects);
});

ipcMain.handle("deleteProject", async (_e, project: string) => {
  const projFile = await getProjectsFileData();
  if (projFile.projects[project] !== undefined) {
    const dataName = projFile.projects[project];
    await fs.unlink(p(dataDirPath, dataName));
    delete projFile.projects[project];
    await fs.writeFile(projectsFilePath, JSON.stringify(projFile));
  }
});

ipcMain.handle("addProject", async (_e, project: string) => {
  const projFile = await getProjectsFileData();
  const
  projFile.projects[project] = dataID
});

ipcMain.ipcMain.handle(
  "parseCSV",
  (_e, rawData: string) =>
    new Promise<CSVData>((res, rej) => {
      let headers: string[];
      const data: DataItem[] = [];
      const parser = csv();
      parser.on("data", (newData) => {
        data.push(newData);
      });
      parser.on("headers", (newHeaders: string[]) => {
        headers = newHeaders;
      });
      parser.on("end", () => {
        console.log("ended");
        res({ headers, data });
      });
      parser.on("error", (err) => {
        rej(err);
      });
      parser.write(rawData);
      parser.end();
    })
);

ipcMain.handle(
  "getFields",
  async (_e, template: string): Promise<string[]> => {
    if (template === "") {
      return [];
    } else {
      const rawData = await pdftk
        .input(p(templateDirPath, template))
        .dumpDataFields()
        .output();
      return rawData
        .toString()
        .split(/\r\n|\n|\r/)
        .filter((line) => line.startsWith("FieldName"))
        .map((fieldName) =>
          fieldName.substring(fieldName.indexOf(":") + 1).trim()
        );
    }
  }
);

const createWindow = async (): Promise<void> => {
  await checkDataDir();

  const mainWindow = new BrowserWindow({
    height: 600,
    width: 800,
    webPreferences: {
      contextIsolation: true,
      preload: p(__dirname, "./preload"),
    },
  });

  mainWindow.removeMenu();
  if (process.env.NODE_ENV === "DEV") mainWindow.webContents.openDevTools();

  mainWindow.loadFile(
    p(__dirname, "../node_modules/@hattmo/pdfgenfe/static/index.html")
  );
};

app.on("ready", createWindow);

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});
