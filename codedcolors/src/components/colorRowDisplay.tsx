import React from "react";

interface IProps {
  colorsSelected: string[];
}

export default ({ colorsSelected }: IProps) => {
  return (
    <div className="colorRow">
      {colorsSelected.map((color, index) => {
        return <div key={index} className={`${color} colorSelector`} />;
      })}
    </div>
  );
};
