import React, { useState } from "react";
import styled from "styled-components";
import { useGetHeaders, useSetHeaders } from "../DataContext";
import deleteImg from "../table/delete.png";
import FormStyle from "./FormStyle";
interface Props {
  onSubmit: () => void;
}
const HeaderForm = ({ onSubmit }: Props) => {
  const headers = useGetHeaders();
  const setHeaders = useSetHeaders();
  const [editHeaders, setEditHeaders] = useState(headers);
  return (
    <HeaderPane>
      {editHeaders.map((header, index) => {
        return (
          <React.Fragment key={index}>
            <input
              type="text"
              value={header}
              onChange={(e) => {
                const tempHeaders = [...editHeaders];
                tempHeaders[index] = e.target.value;
                setEditHeaders(tempHeaders);
              }}
            />
            <div
              onClick={() => {
                setEditHeaders(editHeaders.filter((_, i) => i !== index));
              }}
            >
              <img height="15px" src={deleteImg} alt="" />
            </div>
          </React.Fragment>
        );
      })}
      <input
        onClick={() => {
          setEditHeaders((val) => [...val, ""]);
        }}
        type="button"
        value="Add"
      />
      <input
        onClick={() => {
          setHeaders(editHeaders);
          onSubmit();
        }}
        type="button"
        value="Save"
      />
    </HeaderPane>
  );
};

const HeaderPane = styled(FormStyle)`
  grid-template-columns: 250px auto;
`;

export default HeaderForm;
