import React, { useState } from "react";
import SelectCal from "../selectCal";
import InputCalList from "./inputCalList";
import addIcon from "../../../assets/add.png";
import { ISettingInputItem, ISettingCalendar } from "../../../types";

interface IProps extends React.HTMLAttributes<HTMLDivElement> {
    calendarList: ISettingCalendar[];
    inputItems: ISettingInputItem[];
    setInputItems: (newItems: ISettingInputItem[]) => void;
}
const component = ({ calendarList, inputItems, setInputItems, style, ...rest }: IProps) => {
    const [selectedInputCal, setSelectedInputCal] = useState<ISettingCalendar>();
    return (
        <div style={{ ...defaultStyle, ...style }} {...rest}>
            <SelectCal
                style={{ gridArea: "input" }}
                setSelectedCal={setSelectedInputCal}
                selectedCal={selectedInputCal ?? calendarList[0]}
                cals={calendarList} />
            <div style={{
                cursor: "pointer",
                gridArea: "add",
            }} onClick={() => {
                if (selectedInputCal !== undefined &&
                    inputItems
                        .map((item) => item.cal)
                        .findIndex((item) => item.id === selectedInputCal.id) === -1) {
                    setInputItems([{
                        cal: selectedInputCal,
                        exclude: false,
                        regex: "",
                    }, ...inputItems]);
                }
            }}><img style={{ width: "20px" }} src={addIcon} /></div>
            <InputCalList style={{ gridArea: "items" }} inputItems={inputItems} setInputItems={setInputItems} />
        </div >
    );
};

const defaultStyle: React.CSSProperties = {
    display: "grid",
    gap: "4px",
    gridTemplateColumns: "400px min-content",
    gridTemplateAreas: `"input add"
                        "items items"`,
};

component.displayName = "InputCalBlock";
export default component;