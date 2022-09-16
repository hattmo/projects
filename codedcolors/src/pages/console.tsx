import React, { useState } from "react";
import "../css/console.css";
import IAnswer from "../utility/IAnswer";
import { checkPassword, getAnswers } from "../utility/tester";

interface IProps {
  movePage: () => void;
  flight: string;
  updateFlight: (string) => void;
  setAnswer: (answer: IAnswer) => void;
}

export default ({ movePage, flight, updateFlight, setAnswer }: IProps) => {
  const [isFlightValid, setIsFlightValid] = useState(false);
  const [isPassValid, setIsPassValid] = useState(false);
  const [pass, setPass] = useState("");
  const validateInput = (input: string) => {
    setIsFlightValid(/(A|B|F|C)[0-9][0-9]/.test(input));
  };
  const validatePassword = async (newPass: string) => {
    setIsPassValid(await checkPassword(newPass));
  };
  return (
    <div className="console">
      <div className="consoleTitle">TLP 1<br />CODED COLORS</div>
      <input placeholder="Flight" value={flight}
        className={`consoleFlight ${isFlightValid ? "consoleGood" : "consoleBad"}`} type="text"
        onChange={(e) => {
          if (e.target.value.length < 4) {
            updateFlight(e.target.value);
            validateInput(e.target.value);
          }
        }}></input>
      <input placeholder="Password" value={pass}
        className={`consolePassword ${isPassValid ? "consoleGood" : "consoleBad"}`} type="password"
        onChange={(e) => {
          setPass(e.target.value);
          validatePassword(e.target.value);
        }}></input>
      <div className={`consoleButton ${isFlightValid && isPassValid ? "visibleButton" : "invisibleButton"}`}
        onClick={async (_e) => {
          setAnswer(await getAnswers(pass));
          movePage();
        }}>Begin</div>
    </div>
  );
};
