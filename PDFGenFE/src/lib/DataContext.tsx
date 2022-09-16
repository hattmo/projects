import React, { useContext, useEffect, useState } from "react";
import { useLoadLastData } from "./utility/apiBridge";

interface Context {
  data: DataItem[];
  headers: string[];
  globalData: DataItem;
  template: string;
  project: string;
  setData: React.Dispatch<React.SetStateAction<DataItem[]>>;
  setHeaders: React.Dispatch<React.SetStateAction<string[]>>;
  setGlobalData: React.Dispatch<React.SetStateAction<DataItem>>;
  setTemplate: React.Dispatch<React.SetStateAction<string>>;
  setProject: React.Dispatch<React.SetStateAction<string>>;
  triggerPersist: () => void;
}

interface Props {}

const ctx = React.createContext<Context>({
  data: [],
  headers: [],
  globalData: {},
  template: "",
  project: "",
  setData: () => {},
  setHeaders: () => {},
  setGlobalData: () => {},
  setTemplate: () => {},
  setProject: () => {},
  triggerPersist: () => {},
});

const DataContext = ({ children }: React.PropsWithChildren<Props>) => {
  const {
    data: initialData,
    headers: initialHeaders,
    globalData: initialGlobalData,
    template: initialTemplate,
    project: initialProject,
  } = useLoadLastData();
  const [data, setData] = useState<DataItem[]>(initialData);
  const [headers, setHeaders] = useState<string[]>(initialHeaders);
  const [globalData, setGlobalData] = useState<DataItem>(initialGlobalData);
  const [template, setTemplate] = useState<string>(initialTemplate);
  const [project, setProject] = useState<string>(initialProject);
  const triggerPersist = usePersist(project, {
    data,
    globalData,
    headers,
    template,
  });
  return (
    <ctx.Provider
      value={{
        data,
        headers,
        globalData,
        template,
        project,
        setData,
        setHeaders,
        setGlobalData,
        setTemplate,
        setProject,
        triggerPersist,
      }}
    >
      {children}
    </ctx.Provider>
  );
};

const usePersist = (project: string, data: PersistentData) => {
  const [needsPersist, setNeedsPersist] = useState(false);
  const [timeoutHandle, setTimeoutHandle] = useState<number>(0);
  useEffect(() => {
    if (needsPersist) {
      window.clearTimeout(timeoutHandle);
      setTimeoutHandle(
        window.setTimeout(() => {
          api.savePersistentData(project, data);
        }, 3000)
      );
      setNeedsPersist(false);
    }
  }, [needsPersist, data, timeoutHandle, project]);
  const triggerPersist = () => {
    setNeedsPersist(true);
  };
  return triggerPersist;
};

export const useGetHeaders = () => useContext(ctx).headers;

export const useSetHeaders = () => {
  const { setHeaders, triggerPersist } = useContext(ctx);
  return (val: string[]) => {
    setHeaders(val);
    triggerPersist();
  };
};

export const useGetData = () => useContext(ctx).data;

export const useDeleteData = () => {
  const { setData, triggerPersist } = useContext(ctx);
  return (index: number) => {
    setData((prev) => {
      return prev.filter((_, i) => i !== index);
    });
    triggerPersist();
  };
};

export const useUpdateData = () => {
  const { setData, triggerPersist } = useContext(ctx);
  return (value: DataItem, index: number | undefined) => {
    setData((prev) => {
      const i = index ?? prev.length;
      return [...prev.slice(0, i), value, ...prev.slice(i + 1, prev.length)];
    });
    triggerPersist();
  };
};

export const useAppendData = () => {
  const { setData, triggerPersist } = useContext(ctx);
  return (value: DataItem[]) => {
    setData((prev) => {
      return [...prev, ...value];
    });
    triggerPersist();
  };
};

export const useClearData = () => {
  const { setData, triggerPersist } = useContext(ctx);
  return () => {
    setData([]);
    triggerPersist();
  };
};

export const useGetGlobalData = () => useContext(ctx).globalData;

export const useSetGlobalData = () => {
  const { setGlobalData, globalData, triggerPersist } = useContext(ctx);
  return (key: string, value: string) => {
    setGlobalData({ ...globalData, [key]: value });
    triggerPersist();
  };
};

export const useDeleteGlobalData = () => {
  const { setGlobalData, globalData, triggerPersist } = useContext(ctx);
  return (key: string) => {
    setGlobalData({ ...globalData, [key]: undefined });
    triggerPersist();
  };
};

export const useGetProject = () => useContext(ctx).project;

export const useSetProject = () => {
  const { setProject, triggerPersist } = useContext(ctx);
  return (val: string) => {
    setProject(val);
    triggerPersist();
  };
};

export const useGetTemplate = () => useContext(ctx).template;

export const useSetTemplate = () => {
  const { setHeaders, triggerPersist } = useContext(ctx);
  return (val: string[]) => {
    setHeaders(val);
    triggerPersist();
  };
};


export default DataContext;
