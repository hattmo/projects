type DataItem = { [x: string]: string | undefined };
type CSVData = { headers: string[]; data: DataItem[] };
type PersistentData = {
  data: DataItem[];
  headers: string[];
  globalData: DataItem;
  template: string;
};
type ProjectsFile = {
  current: string;
  projects: {
    [x: string]: string;
  };
};
