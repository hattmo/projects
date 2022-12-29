import React, { useState } from "react";
import styled from "styled-components";
import { useSetGlobalData } from "../DataContext";
import FormStyle from "./FormStyle";
interface Props {
  onSubmit: () => void;
}
const GlobalStateForm = ({ onSubmit }: Props) => {
  const [newState, setNewState] = useState("");
  const setGlobalState = useSetGlobalData();
  return (
    <Pane>
      <input
        value={newState}
        onChange={(e) => {
          setNewState(e.target.value);
        }}
      />
      <input
        type="button"
        value="Add global value"
        onClick={() => {
          setGlobalState(newState, "");
          onSubmit();
        }}
      />
    </Pane>
  );
};

const Pane = styled(FormStyle)``;
export default GlobalStateForm;
