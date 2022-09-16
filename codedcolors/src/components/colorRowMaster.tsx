import React from "react";

interface IProps {
  colorsSelected: string[];
  rotated: boolean;
}

export default ({ colorsSelected, rotated }: IProps) => {
  const colors = colorsSelected.map((color, index) => {
    return <div key={index} className={`${color} colorSelector`} />;
  });
  if (rotated) {
    colors.reverse();
  }
  return (
    <div className="masterRow">
      {colors}
    </div>
  );
};
