import { ipcRenderer, contextBridge } from "electron";

contextBridge.exposeInMainWorld("api", {
  parseCSV: (rawData: string): Promise<CSVData> =>
    ipcRenderer.invoke("parseCSV", rawData),
  getFields: (pdfPath: string): Promise<string> =>
    ipcRenderer.invoke("getFields", pdfPath),
});
