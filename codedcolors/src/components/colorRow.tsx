import React from "react";
import ColorSelector from "./colorSelector";

interface IProps {
  colorsSelected: string[];
  onColorsSelected: (newColors: string[]) => void;
}

export default ({ colorsSelected, onColorsSelected }: IProps) => {
  return (
    <div className="colorRow">
      {colorsSelected.map((color, index) => {
        return <ColorSelector key={index} selectedColor={color} colorClicked={(colorPicked) => {
          const newColors = [...colorsSelected];
          newColors[index] = colorPicked;
          onColorsSelected(newColors);
        }} />;
      })}
    </div>
  );
};
