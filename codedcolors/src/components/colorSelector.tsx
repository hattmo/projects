import React, { useState } from "react";

interface IProps {
  selectedColor: string;
  colorClicked: (clickedColor: string) => void;
}

export default ({ selectedColor, colorClicked }: IProps) => {
  const [isSelectorOpen, setSelectorState] = useState(false);
  if (isSelectorOpen) {
    return (
      <div className="colorSelector">
        <div className="selectorContainer pointer">
          <div className="colorSelector red"
            onClick={(_e) => { colorClicked("red"); setSelectorState(false); }} />
          <div className="colorSelector orange"
            onClick={(_e) => { colorClicked("orange"); setSelectorState(false); }} />
          <div className="colorSelector yellow"
            onClick={(_e) => { colorClicked("yellow"); setSelectorState(false); }} />
          <div className="colorSelector green"
            onClick={(_e) => { colorClicked("green"); setSelectorState(false); }} />
          <div className="colorSelector blue"
            onClick={(_e) => { colorClicked("blue"); setSelectorState(false); }} />
          <div className="colorSelector white"
            onClick={(_e) => { colorClicked("white"); setSelectorState(false); }} />
          <div className="colorSelector black"
            onClick={(_e) => { colorClicked("black"); setSelectorState(false); }} />
          <div className="colorSelector brown"
            onClick={(_e) => { colorClicked("brown"); setSelectorState(false); }} />
        </div>
      </div >
    );
  } else {
    return (
      <div className={`${selectedColor} colorSelector pointer`} onClick={(_e) => setSelectorState(true)} />
    );
  }
};
