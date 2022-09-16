import React, { useState } from "react";
import ColorRowMaster from "../components/colorRowMaster";
import "../css/master.css";
import { checkMaster } from "../utility/tester";
import IAnswer from "../utility/IAnswer";

interface IProps {
  subsetA: string[];
  subsetB: string[];
  subsetC: string[];
  subsetD: string[];
  win: () => void;
  answer: IAnswer;
}

export default ({ subsetA, subsetB, subsetC, subsetD, win, answer }: IProps) => {
  const [ARotated, setARotated] = useState(false);
  const [BRotated, setBRotated] = useState(false);
  const [CRotated, setCRotated] = useState(false);
  const [DRotated, setDRotated] = useState(false);
  const [positions, setPositions] = useState(["A", "B", "C", "D"]);
  const [masterChecks, setMasterChecks] = useState(3);

  const getPositionClass = (subset: string) => {
    const index = positions.findIndex((val) => val === subset);
    switch (index) {
      case 0:
        return "one";
      case 1:
        return "two";
      case 2:
        return "three";
      case 3:
        return "four";
      default:
        return "ERROR";
    }
  };
  const movePosition = (subset: string, direction: string) => {
    const index = positions.findIndex((val) => val === subset);
    if (direction === "left") {
      if (index === 0) {
        return;
      }
      const tempPositions = [...positions];
      const tempValue = tempPositions[index];
      tempPositions[index] = tempPositions[index - 1];
      tempPositions[index - 1] = tempValue;
      setPositions(tempPositions);
    } else {
      if (index === 3) {
        return;
      }
      const tempPositions = [...positions];
      const tempValue = tempPositions[index];
      tempPositions[index] = tempPositions[index + 1];
      tempPositions[index + 1] = tempValue;
      setPositions(tempPositions);
    }
  };
  const getReverseArray = () => {
    return positions.map((val) => {
      switch (val) {
        case "A":
          return ARotated;
        case "B":
          return BRotated;
        case "C":
          return CRotated;
        case "D":
          return DRotated;
        default:
          return true;
      }
    });
  };
  return (
    <div className="master">
      <div className={`masterSubset ${getPositionClass("A")}`}>
        <p className="masterSubsetTitle">Subset A</p>
        <ColorRowMaster colorsSelected={subsetA} rotated={ARotated} />
        <p className="masterButtons" onClick={(_e) => { movePosition("A", "left"); }}>⬅️</p>
        <p className="masterButtons" onClick={(_e) => {
          if (ARotated) {
            setARotated(false);
          } else {
            setARotated(true);
          }
        }}>🔄</p>
        <p className="masterButtons" onClick={(_e) => { movePosition("A", "right"); }}>➡️</p>
      </div>
      <div className={`masterSubset ${getPositionClass("B")}`}>
        <p className="masterSubsetTitle">Subset B</p>
        <ColorRowMaster colorsSelected={subsetB} rotated={BRotated} />
        <p className="masterButtons" onClick={(_e) => { movePosition("B", "left"); }}>⬅️</p>
        <p className="masterButtons" onClick={(_e) => {
          if (BRotated) {
            setBRotated(false);
          } else {
            setBRotated(true);
          }
        }}>🔄</p>
        <p className="masterButtons" onClick={(_e) => { movePosition("B", "right"); }}>➡️</p>
      </div>
      <div className={`masterSubset ${getPositionClass("C")}`}>
        <p className="masterSubsetTitle">Subset C</p>
        <ColorRowMaster colorsSelected={subsetC} rotated={CRotated} />
        <p className="masterButtons" onClick={(_e) => { movePosition("C", "left"); }}>⬅️</p>
        <p className="masterButtons" onClick={(_e) => {
          if (CRotated) {
            setCRotated(false);
          } else {
            setCRotated(true);
          }
        }}>🔄</p>
        <p className="masterButtons" onClick={(_e) => { movePosition("C", "right"); }}>➡️</p>
      </div>
      <div className={`masterSubset ${getPositionClass("D")}`}>
        <p className="masterSubsetTitle">Subset D</p>
        <ColorRowMaster colorsSelected={subsetD} rotated={DRotated} />
        <p className="masterButtons" onClick={(_e) => { movePosition("D", "left"); }}>⬅️</p>
        <p className="masterButtons" onClick={(_e) => {
          if (DRotated) {
            setDRotated(false);
          } else {
            setDRotated(true);
          }
        }}>🔄</p>
        <p className="masterButtons" onClick={(_e) => { movePosition("D", "right"); }}>➡️</p>
      </div>
      <p className="masterCheck" onClick={(_e) => {
        if (masterChecks > 0) {
          setMasterChecks(masterChecks - 1);
          if (checkMaster(positions, getReverseArray(), answer)) {
            win();
          }
        }
      }}>
        {`Check Master Sequence (${masterChecks})`}
      </p>
    </div>
  );
};
