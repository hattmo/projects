import React, { useState } from "react";
import ColorRequest from "./colorRequest";
import MasterSequence from "./masterSequence";
import Subsets from "./subsets";
import Console from "./console";
import PreBrief from "./prebrief";
import ScoreScreen from "./scoreScreen";
import ReactModal from "react-modal";
import "../css/failmodal.css";
import "../css/winmodal.css";
import IAnswer from "../utility/IAnswer";

export default () => {
  const [flight, updateFlight] = useState("");
  const [currentPage, setCurrentPage] = useState("console");
  const [subsetA, updateA] = useState(["red", "red", "red", "red"]);
  const [subsetB, updateB] = useState(["red", "red", "red", "red"]);
  const [subsetC, updateC] = useState(["red", "red", "red", "red"]);
  const [subsetD, updateD] = useState(["red", "red", "red", "red"]);
  const [startTime, updateStartTime] = useState(new Date());
  const [endTime, updateEndTime] = useState(new Date());
  const [timerHandle, setTimerHandle] = useState(0);
  const [isFailed, setIsFailed] = useState(false);
  const [isWon, setIsWon] = useState(false);
  const [results, setResults] = useState(false);
  const [events, setEvents] = useState<Array<{ event: string, time: number }>>([]);
  const [answer, setAnswer] = useState<IAnswer>({
    color: [
      ["red", "red", "red", "red"],
      ["red", "red", "red", "red"],
      ["red", "red", "red", "red"],
      ["red", "red", "red", "red"],
    ],
    master: [
      { order: ["A", "B", "C", "D"], reverse: [false, false, false, false] },
      { order: ["A", "B", "C", "D"], reverse: [false, false, false, false] },
      { order: ["A", "B", "C", "D"], reverse: [false, false, false, false] },
      { order: ["A", "B", "C", "D"], reverse: [false, false, false, false] },
    ],
  });

  const startTimer = () => {
    endTimer();
    updateStartTime(new Date());
    setTimerHandle(window.setTimeout(() => {
      updateEndTime(new Date());
      setIsFailed(true);
      window.setTimeout(() => {
        setIsFailed(false);
        setResults(false);
        setCurrentPage("scorescreen");
      }, 3000);
    }, 900000));
  };
  const endTimer = () => {
    updateEndTime(new Date());
    clearTimeout(timerHandle);
  };
  const updateColors = (subset: string, colors: string[]) => {
    switch (subset) {
      case "A":
        updateA(colors);
        return;
      case "B":
        updateB(colors);
        return;
      case "C":
        updateC(colors);
        return;
      case "D":
        updateD(colors);
        return;
    }
  };
  let body;
  switch (currentPage) {
    case "console":
      body = <Console flight={flight} updateFlight={updateFlight}
        movePage={() => setCurrentPage("colorrequest")} setAnswer={setAnswer} />;
      break;
    case "colorrequest":
      body = <ColorRequest updateRow={updateColors}
        subsetA={subsetA} subsetB={subsetB} subsetC={subsetC} subsetD={subsetD} onSubmit={() => {
          setCurrentPage("prebrief");
        }} />;
      break;
    case "prebrief":
      body = <PreBrief onBack={() => {
        setCurrentPage("colorrequest");
      }} onBegin={() => {
        setCurrentPage("subsets");
      }} />;
      break;
    case "subsets":
      body = <Subsets subsetA={subsetA} subsetB={subsetB} subsetC={subsetC} subsetD={subsetD}
        updateRow={updateColors} startTimer={startTimer} isFailed={isFailed}
        moveToMaster={() => { setCurrentPage("master"); }} setEvents={setEvents} answer={answer} />;
      break;
    case "master":
      body = <MasterSequence win={() => {
        setIsWon(true);
        endTimer();
        setEvents((prevState) => [...prevState, { event: "TLP WIN", time: (new Date()).getTime() }]);
        setTimeout(() => {
          setIsWon(false);
          setResults(true);
          setCurrentPage("scorescreen");
        }, 3000);
      }} subsetA={subsetA} subsetB={subsetB} subsetC={subsetC} subsetD={subsetD}
        answer={answer} />;
      break;
    case "scorescreen":
      body = <ScoreScreen results={results}
        startTime={startTime.getTime()} endTime={endTime.getTime()}
        events={events} flight={flight} />;
      break;
    default:
      body = <div>ERROR</div>;
  }
  return (
    <React.Fragment>
      {body}
      <ReactModal className="failmodal" overlayClassName="failoverlay" isOpen={isFailed} ariaHideApp={false}>
        <div className="failblock">
          <p className="failtitle">MISSION INCOMPLETE</p>
        </div>
      </ReactModal>
      <ReactModal className="winmodal" overlayClassName="winoverlay" isOpen={isWon} ariaHideApp={false}>
        <div className="winblock">
          <p className="wintitle">MISSION COMPLETE</p>
        </div>
      </ReactModal>
    </React.Fragment>

  );
};
