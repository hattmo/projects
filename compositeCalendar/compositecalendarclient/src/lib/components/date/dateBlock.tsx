import React from "react";

interface IProps extends React.HTMLAttributes<HTMLDivElement> {
    startDate: string;
    setStartDate: (string) => void;
    endDate: string;
    setEndDate: (string) => void;
}

const component = ({ startDate, setStartDate, endDate, setEndDate, ...rest }: IProps) => {

    return (
        <div {...rest}>
            <div>From:</div>
            <input type="date" value={startDate} onChange={(e) => { setStartDate(e.currentTarget.value); }} />
            <div>To:</div>
            <input type="date" value={endDate} onChange={(e) => { setEndDate(e.currentTarget.value); }} />
        </div>
    );
};

component.displayName = "DateBlock";
export default component;