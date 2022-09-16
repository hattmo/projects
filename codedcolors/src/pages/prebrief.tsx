import React, { useState } from "react";
import "../prebrief.webm";
import "../css/prebrief.css";

interface IProps {
  onBack: () => void;
  onBegin: () => void;
}

export default ({ onBack, onBegin }: IProps) => {
  const [paused, setPaused] = useState(true);
  return (
    <div className="prebrief">
      <video className="prebriefVideo" id="video" src="video/prebrief.webm" >video not loaded</video>
      <p className="prebriefButtons back" onClick={(_e) => { onBack(); }}>Back</p>
      <p className="prebriefButtons play" onClick={(_e) => {
        const video = document.getElementById("video") as HTMLVideoElement;
        if (video.paused) {
          setPaused(false);
          video.play();
        } else {
          setPaused(true);
          video.pause();
        }
      }}>{paused ? "Play" : "Pause"}</p>
      <p className="prebriefButtons begin" onClick={(_e) => { onBegin(); }}>Begin TLP</p>
    </div>
  );
};
