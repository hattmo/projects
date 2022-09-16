import React from "react";
import "../css/results.css";
interface IProps {
  results: boolean;
  startTime: number;
  endTime: number;
  events: Array<{
    event: string,
    time: number,
  }>;
  flight: string;
}

export default ({ results, startTime, endTime, events, flight }: IProps) => {
  const submitResults = async () => {
    try {
      const result = await fetch("/submit", {
        method: "POST",
        body: JSON.stringify({
          flight,
          time: endTime - startTime,
        }),
      });
      console.log(await result.text());
    } catch (err) {
      console.log('Offline');
    }
  };
  submitResults();
  return (
    <div className="results">
      <p className="resultsTitle">{results ? "MISSION COMPLETE" : "MISSION INCOMPLETE"}</p>
      <p className="resultsTime">{`${formatTime(endTime - startTime)}`}</p>
      <div className="resultEvents">
        <p className="resultEvent">
          {"00:00 : TLP START"}
        </p>
        {events.map((val, index) => {
          return (
            <p key={index} className="resultEvent">
              {`${formatTime(val.time - startTime)} : ${val.event}`}
            </p>
          );
        })}
        <p className="resultEvent">
          {"15:00 : TLP END"}
        </p>
      </div>
    </div>
  );
};

function formatTime(time: number): string {
  let seconds = Math.floor(time / 1000);
  const minutes = Math.floor(seconds / 60);
  seconds = seconds % 60;
  return `${minutes < 10 ? "0" : ""}${minutes}:${seconds < 10 ? "0" : ""}${seconds}`;

}
