import { useEffect, useState } from "react";

export const useLoadLastData = () => {
  const [persistentData, setPersistentData] = useState<PersistentData>({
    data: [],
    headers: [],
    globalData: {},
    template: "",
  });
  const [project, setInitialProject] = useState<string>("");
  useEffect(() => {
    (async () => {
      const proj = await api.getLastProject();
      setPersistentData(await api.loadPersistentData(proj));
      setInitialProject(proj);
    })();
  }, []);
  return { ...persistentData, project };
};

export const useGetProjects = () => {
  const [projects, setProjects] = useState<string[]>([]);
  useEffect(() => {
    api.getProjects().then((val) => {
      setProjects(val);
    });
  }, []);
  return projects;
};

export const useGetFields = (template: string) => {
  const [fields, setFields] = useState<string[]>([]);
  useEffect(() => {
    api.getFields(template).then((val) => {
      setFields(val);
    });
  }, [template]);
  return fields;
};
