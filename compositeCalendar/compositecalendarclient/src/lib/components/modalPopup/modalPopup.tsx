import React, { useEffect, useState } from "react";

interface IProps extends React.HTMLAttributes<HTMLDivElement> {
    message: string;
    closeModal: () => void;
}

const component = ({ message,closeModal, style, ...rest }: IProps) => {
    const [top, setTop] = useState("0px");
    const [opacity, setOpacity] = useState("0");
    useEffect(() => {
        setTop("20px");
        setOpacity("1");
        setTimeout(() => {
            closeModal();
        }, 20000);
    }, []);
    return (
        <div style={{ ...defaultStyle, ...style }} {...rest}>
            <div style={{ top, opacity, ...popupStyle }}>
                <div>
                    {message}
                </div>
                <div onClick={closeModal} style={buttonStyle}>
                    OK
                </div>
            </div>
        </div>
    );
};

const defaultStyle: React.CSSProperties = {
    pointerEvents: "none",
    position: "fixed",
    display: "grid",
    top: "0",
    left: "0",
    width: "100%",
    height: "100%",
};

const popupStyle: React.CSSProperties = {
    pointerEvents: "auto",
    display: "grid",
    backgroundColor: "lavender",
    padding: "10px",
    gap: "10px",
    justifySelf: "center",
    alignSelf: "start",
    transitionProperty: "all",
    transitionDuration: "1s",
    transitionTimingFunction: "ease",
    position: "relative",
    border: "1px solid black",
    borderRadius: "8px",
    placeItems: "center",
};

const buttonStyle: React.CSSProperties = {
    border: "1px solid black",
    borderRadius: "8px",
    cursor: "pointer",
    textAlign: "center",
    padding: "2px 8px",
    backgroundColor: "white",
};

component.displayName = "ModalPopup";
export default component;