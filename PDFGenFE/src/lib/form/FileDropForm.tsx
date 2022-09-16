import React, { useState } from "react";
import styled from "styled-components";
import { useAppendData, useGetHeaders } from "../DataContext";
import FormStyle from "./FormStyle";

interface Props {
  onSubmit: () => void;
  initialHeaders: string[];
  initialData: DataItem[];
}

const FileDropForm = ({ onSubmit, initialHeaders, initialData }: Props) => {
  const appendData = useAppendData();
  const currHeaders = useGetHeaders();
  const [availHead, unMatchedHead] = findMismatch(currHeaders, initialHeaders);
  const [mapping, setMapping] = useState(defaultMapping(unMatchedHead));
  if (availHead.length === 0) {
    appendData(initialData);
    onSubmit();
  }
  return (
    <FileDropPane>
      {unMatchedHead.map((item, i) => {
        return (
          <React.Fragment>
            <div key={i}>{item}</div>
            <select
              value={mapping[item]}
              onChange={(e) => {
                setMapping({ ...mapping, [item]: e.target.value });
              }}
            >
              {availHead
                .filter(
                  (availChoice) =>
                    !(
                      mapping[item] !== availChoice &&
                      Object.values(mapping).includes(availChoice)
                    )
                )
                .map((availChoice, i) => {
                  return (
                    <option key={i} value={availChoice}>
                      {availChoice}
                    </option>
                  );
                })}
              <option value={"None"}>None</option>
            </select>
          </React.Fragment>
        );
      })}
      <FormButton
        onClick={() => {
          appendData(
            initialData.map<DataItem>((dataItem) => {
              const mapped = {};
              initialHeaders.forEach((key) => {
                if (unMatchedHead.includes(key)) {
                  if (mapping[key] !== "None") {
                    mapped[mapping[key]] = dataItem[key];
                  }
                } else {
                  mapped[key] = dataItem[key];
                }
              });
              return mapped;
            })
          );
          onSubmit();
        }}
        type="button"
        value="ok"
      />
    </FileDropPane>
  );
};

const defaultMapping = (unmatched: string[]) => {
  const out = {};
  unmatched.forEach((item) => {
    out[item] = "None";
  });
  return out;
};

const findMismatch = (curr: string[], initial: string[]) => [
  curr.filter((item) => !initial.includes(item)),
  initial.filter((item) => !curr.includes(item)),
];

const FileDropPane = styled(FormStyle)`
  grid-template-columns: auto auto;
`;

const FormButton = styled.input`
  grid-column: auto / span 2;
  place-self: center;
`;

export default FileDropForm;
