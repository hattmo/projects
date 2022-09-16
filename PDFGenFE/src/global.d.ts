type DataItem = { [x: string]: string | undefined };
type CSVData = { headers: string[]; data: DataItem[] };
type PersistentData = {
  data: DataItem[];
  headers: string[];
  globalData: DataItem;
  template: string;
};
declare namespace api {
  function getLastProject(): Promise<string>;

  function getProjects(): Promise<string[]>;
  function deleteProject(project: string): Promise<void>;
  function addProject(project: string): Promise<void>;

  function getTemplates(): Promise<string[]>;
  function deleteTemplate(template: string): Promise<void>;
  function addTemplate(template: string, data: ArrayBuffer): Promise<void>;

  function loadPersistentData(project: string): Promise<PersistentData>;
  function savePersistentData(
    project: string,
    data: PersistentData
  ): Promise<void>;

  function getFields(template: string): Promise<string[]>;

  function parseCSV(rawData: string): Promise<CSVData>;
}
