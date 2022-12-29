import React from "react";
import ColorRow from "../components/colorRow";
import "../css/colorRequest.css";

interface IProps {
  subsetA: string[];
  subsetB: string[];
  subsetC: string[];
  subsetD: string[];
  updateRow: (subset: string, value: string[]) => void;
  onSubmit: () => void;
}

export default ({ subsetA, subsetB, subsetC, subsetD, updateRow, onSubmit }: IProps) => {
  return (
    <div className="colorRequest">
      <p className="colorRequestHeader">Color Request Submission</p>
      <div className="colorRequestSubset subsetA">
        <p className="colorRequestSubsetTitle">Subset A</p>
        <ColorRow colorsSelected={subsetA} onColorsSelected={(newColors) => { updateRow("A", newColors); }} />
      </div>
      <div className="colorRequestSubset subsetB">
        <p className="colorRequestSubsetTitle">Subset B</p>
        <ColorRow colorsSelected={subsetB} onColorsSelected={(newColors) => { updateRow("B", newColors); }} />
      </div>
      <div className="colorRequestSubset subsetC">
        <p className="colorRequestSubsetTitle">Subset C</p>
        <ColorRow colorsSelected={subsetC} onColorsSelected={(newColors) => { updateRow("C", newColors); }} />
      </div >
      <div className="colorRequestSubset subsetD">
        <p className="colorRequestSubsetTitle">Subset D</p>
        <ColorRow colorsSelected={subsetD} onColorsSelected={(newColors) => { updateRow("D", newColors); }} />
      </div >
      <p className="colorRequestSubmit" onClick={(_e) => onSubmit()}>
        Submit
        </p>
    </div >
  );
};
