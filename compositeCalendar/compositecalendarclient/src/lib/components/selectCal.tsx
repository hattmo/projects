import React from "react";
import { ISettingCalendar } from "../../types";
interface IProps extends React.HTMLAttributes<HTMLSelectElement> {
    cals: ISettingCalendar[];
    selectedCal: ISettingCalendar;
    setSelectedCal: (item: ISettingCalendar) => void;
}

const component = ({ cals, selectedCal, setSelectedCal, ...rest }: IProps) => {
    if (cals.length !== 0) {
        const selectedCalId = selectedCal?.id ?? cals[0].id;
        return (
            <select value={selectedCalId} onChange={(e) => {
                setSelectedCal(cals.find((item) => {
                    return item.id === e.target.value;
                }) ?? cals[0]);
            }} {...rest}>
                {cals.map((item, index) => {
                    return (
                        <option value={item.id} key={index}>
                            {item.name}
                        </option>
                    );
                })}
            </select>
        );
    } else {
        return (
            <select {...rest}>
                <option>Loading Calendars...</option>
            </select>
        );
    }
};

component.displayName = "SelectCal";
export default component;