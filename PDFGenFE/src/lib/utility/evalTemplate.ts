import { useGetData, useGetGlobalData } from "../DataContext";

export default (input: string, index: number): string => {
  const data = useGetData()[index];
  const globalData = useGetGlobalData();
  return new Function("data", "global", `return \`${input}\``)(
    data,
    globalData
  );
};
