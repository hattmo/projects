import React from "react";
import trash from "../../../assets/trash.png";
import { ISettingInputItem } from "../../../types";
interface IProps extends React.HTMLAttributes<HTMLDivElement> {
    inputItems: ISettingInputItem[];
    setInputItems: (newInput: ISettingInputItem[]) => void;
}

const component = ({ inputItems, setInputItems, style, ...rest }: IProps) => {
    return (
        <div style={{
            display: "grid",
            gap: "4px",
            ...style,
        }} {...rest}>
            {inputItems.map((item, index) => {
                return (
                    <div style={itemStyle} key={index}>
                        <div style={{
                            width: "100%",
                            placeSelf: "start",
                            overflow: "hidden",
                            textOverflow: "ellipsis",
                            whiteSpace: "nowrap",
                        }}>
                            {item.cal.name}
                        </div>
                        <div style={{
                            border: "1px solid black",
                            padding: "2px",
                            fontSize: "8pt",
                            borderRadius: "4px",
                            cursor: "pointer",
                            backgroundColor: (item.exclude ? "lightpink" : "lightgreen"),
                        }} onClick={() => { setInputItems(updateExclude(inputItems, index)); }} >
                            {item.exclude ? "excl." : "incl."}
                        </div>
                        <input type="text" value={item.regex} onChange={(e) => {
                            setInputItems(updateRegex(e, inputItems, index));
                        }} />
                        <div style={{ cursor: "pointer" }}
                            onClick={() => { setInputItems(inputItems.filter((_item, i) => i !== index)); }}>
                            <img style={{ width: "20px" }} src={trash} />
                        </div>
                    </div>
                );
            })}
        </div>
    );
};

const updateExclude = (
    inputItems: ISettingInputItem[],
    matchindex: number,
): ISettingInputItem[] => {
    return inputItems.map((item, index) => {
        if (index === matchindex) {
            return {
                ...item,
                exclude: !item.exclude,
            };
        } else {
            return item;
        }
    });
};

const updateRegex = (
    e: React.ChangeEvent<HTMLInputElement>,
    inputItems: ISettingInputItem[],
    matchindex: number,
): ISettingInputItem[] => {
    const newRegex = e.currentTarget.value;
    return inputItems.map((item, index) => {
        if (index === matchindex) {
            return {
                ...item,
                regex: newRegex,
            };
        } else {
            return item;
        }
    });
};

const itemStyle: React.CSSProperties = {
    gap: "4px",
    display: "grid",
    gridTemplateColumns: "auto min-content min-content min-content",
    placeItems: "center",
};

component.displayName = "InputCalList";
export default component;