import React, { useState } from "react";
import styled from "styled-components";
import { useGetData, useGetHeaders, useUpdateData } from "../DataContext";
import FormStyle from "./FormStyle";

interface Props {
  onSubmit: () => void;
  row?: number;
}

const EditForm = ({ onSubmit, row }: Props) => {
  const data = useGetData();
  const updateData = useUpdateData();
  const headers = useGetHeaders();
  const [editData, setEditData] = useState<DataItem>(
    row === undefined ? {} : data[row]
  );

  return (
    <EditPane>
      {headers.map((header) => {
        return (
          <React.Fragment>
            <div>{header}</div>
            <input
              type="text"
              value={editData[header] ?? ""}
              onChange={(e) => {
                setEditData({ ...editData, [header]: e.target.value });
              }}
            />
          </React.Fragment>
        );
      })}
      <FormButton
        className="formButton"
        onClick={() => {
          updateData(editData, row);
          onSubmit();
        }}
        type="button"
        value="save"
      />
    </EditPane>
  );
};

const EditPane = styled(FormStyle)`
  grid-template-columns: auto 250px;
  text-align: right;
`;

const FormButton = styled.input`
  grid-column: auto / span 2;
  place-self: center;
`;

export default EditForm;
