import React, { useState } from "react";
import InputCalBlock from "./inputCal/inputCalBlock";
import DateBlock from "./date/dateBlock";
import SelectCal from "./selectCal";
import ModalPopup from "./modalPopup/modalPopup";
import trash from "../../assets/trash.png";
import { ISettingCalendar, ISetting } from "../../types";

interface IProps extends React.HTMLAttributes<HTMLDivElement> {
    calendarList: ISettingCalendar[];
    writeableCalendarList: ISettingCalendar[];
    logout: (message: string) => void;
    savedSettings: ISetting;
    setSavedSettings: (newSettings: ISetting) => void;
    remove: () => void;
}

const component = ({
    calendarList,
    writeableCalendarList,
    logout,
    savedSettings,
    setSavedSettings,
    remove,
    style,
    ...rest
}: IProps) => {
    const [modalMessage, setModalMessage] = useState("");
    const { inputItems, outputCal, startDate, endDate } = savedSettings;
    const upDateSave = (field: string) => {
        return ((value) => {
            setSavedSettings({
                ...savedSettings,
                [field]: value,
            });
        });
    };
    return (
        <div style={{ ...defaultStyle, ...style }} {...rest}>
            <div style={{ gridArea: "topRow", display: "grid" }}>
                <div style={{ justifySelf: "right", cursor: "pointer" }} onClick={remove}><img style={{ width: "20px" }} src={trash} /></div>
            </div>
            <div style={{ gridArea: "inputT", placeSelf: "center" }}>Input Calendars</div>
            <div style={{ gridArea: "dateT", placeSelf: "center" }}>Date Range</div>
            <div style={{ gridArea: "outputT", placeSelf: "center" }}>Output Calendar</div>
            <InputCalBlock style={{
                placeSelf: "start",
                gridArea: "input",
            }} calendarList={calendarList}
                inputItems={inputItems} setInputItems={upDateSave("inputItems")} />
            <DateBlock style={{ gridArea: "date" }} startDate={startDate} endDate={endDate}
                setStartDate={upDateSave("startDate")} setEndDate={upDateSave("endDate")} />
            <SelectCal style={{
                alignSelf: "start",
                gridArea: "output",
            }} cals={writeableCalendarList}
                selectedCal={outputCal ?? writeableCalendarList[0]} setSelectedCal={upDateSave("outputCal")} />
            {modalMessage !== "" ?
                <ModalPopup message={modalMessage} closeModal={() => { setModalMessage(""); }} /> :
                null}
        </div >
    );
};

const defaultStyle: React.CSSProperties = {
    display: "grid",
    gap: "4px",
    gridTemplateAreas: `"topRow   topRow  topRow"
                        "inputT   dateT   outputT"
                        "input    date    output"`,
    gridTemplateColumns: "min-content min-content min-content",
    padding: "4px",
    border: "1px solid black",
    borderRadius: "5px",
    backgroundColor: "lavender",
};

component.displayName = "Rule";
export default component;