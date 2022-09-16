import React, { useState, useEffect } from "react";
import ColorRowDisplay from "../components/colorRowDisplay";
import ReactModal from "react-modal";
import ColorRow from "../components/colorRow";
import "../css/modal.css";
import "../css/subsetPage.css";
import { checkColors, checkPosition } from "../utility/tester";
import IAnswer from "../utility/IAnswer";
interface IProps {
  subsetA: string[];
  subsetB: string[];
  subsetC: string[];
  subsetD: string[];
  updateRow: (subset: string, value: string[]) => void;
  startTimer: () => void;
  isFailed: boolean;
  moveToMaster: () => void;
  setEvents: (event: any) => void;
  answer: IAnswer;
}

export default ({
  subsetA,
  subsetB,
  subsetC,
  subsetD,
  updateRow,
  startTimer,
  isFailed,
  moveToMaster,
  setEvents,
  answer,
}: IProps) => {
  const [resultMessage, setResultMessage] = useState("");
  const [modalOpen, setIsModalOpen] = useState(false);
  const [selectedSubset, setSelectedSubset] = useState("A");
  const [subsetAColorCount, setSubsetAColorCount] = useState(6);
  const [subsetBColorCount, setSubsetBColorCount] = useState(6);
  const [subsetCColorCount, setSubsetCColorCount] = useState(6);
  const [subsetDColorCount, setSubsetDColorCount] = useState(6);
  const [subsetAPositionCount, setSubsetAPositionCount] = useState(6);
  const [subsetBPositionCount, setSubsetBPositionCount] = useState(6);
  const [subsetCPositionCount, setSubsetCPositionCount] = useState(6);
  const [subsetDPositionCount, setSubsetDPositionCount] = useState(6);
  const [subsetAWin, setSubsetAWin] = useState(false);
  const [subsetBWin, setSubsetBWin] = useState(false);
  const [subsetCWin, setSubsetCWin] = useState(false);
  const [subsetDWin, setSubsetDWin] = useState(false);
  const [subsetWin, setSubsetWin] = useState(false);

  useEffect(() => {
    startTimer();
  }, []);

  if (isFailed && modalOpen) {
    setIsModalOpen(false);
  }
  const decrementColorCount = () => {
    switch (selectedSubset) {
      case "A":
        if (subsetAColorCount <= 0) {
          return false;
        } else {
          setSubsetAColorCount(subsetAColorCount - 1);
          return true;
        }
      case "B":
        if (subsetBColorCount <= 0) {
          return false;
        } else {
          setSubsetBColorCount(subsetBColorCount - 1);
          return true;
        }
      case "C":
        if (subsetCColorCount <= 0) {
          return false;
        } else {
          setSubsetCColorCount(subsetCColorCount - 1);
          return true;
        }
      case "D":
        if (subsetDColorCount <= 0) {
          return false;
        } else {
          setSubsetDColorCount(subsetDColorCount - 1);
          return true;
        }
      default:
        return false;
    }
  };
  const decrementPositionCount = () => {
    switch (selectedSubset) {
      case "A":
        if (subsetAPositionCount <= 0) {
          return false;
        } else {
          setSubsetAPositionCount(subsetAPositionCount - 1);
          return true;
        }
      case "B":
        if (subsetBPositionCount <= 0) {
          return false;
        } else {
          setSubsetBPositionCount(subsetBPositionCount - 1);
          return true;
        }
      case "C":
        if (subsetCPositionCount <= 0) {
          return false;
        } else {
          setSubsetCPositionCount(subsetCPositionCount - 1);
          return true;
        }
      case "D":
        if (subsetDPositionCount <= 0) {
          return false;
        } else {
          setSubsetDPositionCount(subsetDPositionCount - 1);
          return true;
        }
      default:
        return false;
    }
  };
  const getColorCheckCount = () => {
    switch (selectedSubset) {
      case "A":
        return subsetAColorCount;
      case "B":
        return subsetBColorCount;
      case "C":
        return subsetCColorCount;
      case "D":
        return subsetDColorCount;
      default:
        return 0;
    }
  };
  const getPositionCheckCount = () => {
    switch (selectedSubset) {
      case "A":
        return subsetAPositionCount;
      case "B":
        return subsetBPositionCount;
      case "C":
        return subsetCPositionCount;
      case "D":
        return subsetDPositionCount;
      default:
        return 0;
    }
  };
  const getSubset = () => {
    switch (selectedSubset) {
      case "A":
        return subsetA;
      case "B":
        return subsetB;
      case "C":
        return subsetC;
      case "D":
        return subsetD;
      default:
        return subsetA;
    }
  };

  if (subsetAWin && subsetBWin && subsetCWin && subsetDWin && !subsetWin) {
    setSubsetWin(true);
  }

  const setOneSubsetWin = () => {
    switch (selectedSubset) {
      case "A":
        setSubsetAWin(true);
        break;
      case "B":
        setSubsetBWin(true);
        break;
      case "C":
        setSubsetCWin(true);
        break;
      case "D":
        setSubsetDWin(true);
        break;
    }
  };
  const message = subsetWin ? "Solve Master Sequence" : resultMessage;
  return (
    <div className={`subsetPage`}>
      <p className={`subsetPageHeader ${subsetWin ? "win" : ""}`}> Subsets</p>
      <div
        className={`subsetPageSubset subsetA ${subsetAWin ? "win" : ""}`}
        onClick={(_e) => {
          setIsModalOpen(true);
          setSelectedSubset("A");
        }}
      >
        <p className="subsetPageTitle">Subset A</p>
        <ColorRowDisplay colorsSelected={subsetA} />
      </div>
      <div
        className={`subsetPageSubset subsetB ${subsetBWin ? "win" : ""}`}
        onClick={(_e) => {
          setIsModalOpen(true);
          setSelectedSubset("B");
        }}
      >
        <p className="subsetPageTitle">Subset B</p>
        <ColorRowDisplay colorsSelected={subsetB} />
      </div>
      <div
        className={`subsetPageSubset subsetC ${subsetCWin ? "win" : ""}`}
        onClick={(_e) => {
          setIsModalOpen(true);
          setSelectedSubset("C");
        }}
      >
        <p className="subsetPageTitle">Subset C</p>
        <ColorRowDisplay colorsSelected={subsetC} />
      </div>
      <div
        className={`subsetPageSubset subsetD ${subsetDWin ? "win" : ""}`}
        onClick={(_e) => {
          setIsModalOpen(true);
          setSelectedSubset("D");
        }}
      >
        <p className="subsetPageTitle">Subset D</p>
        <ColorRowDisplay colorsSelected={subsetD} />
      </div>
      <p
        className={`subsetpageResultMessage ${
          subsetWin ? "win pointer winMessage" : ""
        }`}
        onClick={(_e) => {
          if (subsetWin) {
            moveToMaster();
          }
        }}
      >
        {message}
      </p>
      <ReactModal
        className="modal"
        overlayClassName="overlay"
        isOpen={modalOpen}
        ariaHideApp={false}
      >
        <div className="modalContainer">
          <p className="modalTitle">{`Subset ${selectedSubset}`}</p>
          <div className="modalRow">
            <ColorRow
              colorsSelected={getSubset()}
              onColorsSelected={(newColors) => {
                updateRow(selectedSubset, newColors);
              }}
            />
          </div>
          <p
            className="modalColors"
            onClick={(_e) => {
              setIsModalOpen(false);
              if (decrementColorCount()) {
                const correctColors = checkColors(
                  getSubset(),
                  selectedSubset,
                  answer
                );
                // tslint:disable-next-line: max-line-length
                setResultMessage(
                  `Subset ${selectedSubset} color, ${correctColors} color card${
                    correctColors === 1 ? "" : "s"
                  } ${
                    correctColors === 1 ? "is" : "are"
                  } correct`
                );
                setEvents((prevState) => [
                  ...prevState,
                  {
                    event: `Subset ${selectedSubset} Color Check`,
                    time: new Date().getTime(),
                  },
                ]);
              } else {
                setResultMessage(`You are out of color checks`);
              }
            }}
          >{`Check Colors (${getColorCheckCount()})`}</p>
          <p
            className="modalPosition"
            onClick={(_e) => {
              setIsModalOpen(false);
              if (decrementPositionCount()) {
                const correctPositions = checkPosition(
                  getSubset(),
                  selectedSubset,
                  answer
                );
                if (correctPositions === 4) {
                  setOneSubsetWin();
                  setEvents((prevState) => [
                    ...prevState,
                    {
                      event: `Subset ${selectedSubset} Complete`,
                      time: new Date().getTime(),
                    },
                  ]);
                } else {
                  setEvents((prevState) => [
                    ...prevState,
                    {
                      event: `Subset ${selectedSubset} Position Check`,
                      time: new Date().getTime(),
                    },
                  ]);
                }
                // tslint:disable-next-line: max-line-length
                setResultMessage(
                  `Subset ${selectedSubset} position, ${correctPositions} card${
                    correctPositions === 1 ? "" : "s"
                  } ${
                    correctPositions === 1 ? "is" : "are"
                  } in the correct position`
                );
              } else {
                setResultMessage(`You are out of position checks`);
              }
            }}
          >
            {" "}
            {`Check Position (${getPositionCheckCount()})`}
          </p>
          <p
            className="modalClose"
            onClick={(_e) => {
              setIsModalOpen(false);
            }}
          >
            Cancel
          </p>
        </div>
      </ReactModal>
    </div>
  );
};
