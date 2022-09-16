import React from "react";
import loginNormal from "../../assets/loginNormal.png";
import { Link } from "react-router-dom";

interface IProps extends React.HTMLAttributes<HTMLDivElement> {
}

const component = ({ style, ...rest }: IProps) => {
    return (
        <div style={{ ...defaultStyle, ...rest }} {...rest}>
            <div style={panelStyle}>
                <div style={{
                    fontSize: "30pt",
                }}>
                    Composite Calendar
                </div >
                <a href={"/login"}>
                    <img style={{ width: "150px" }} src={loginNormal} />
                </a>
                <div style={{
                    display: "grid",
                    gridTemplateColumns: "auto auto",
                    padding: "4px",
                    gap: "4px",
                }}>
                    <Link to="/terms">Terms</Link>
                    <Link to="/privacy">Privacy</Link>
                </div>
            </div>
        </div >
    );
};

const defaultStyle: React.CSSProperties = {
    display: "grid",
    height: "100%",
    placeItems: "center",
};

const panelStyle: React.CSSProperties = {
    display: "grid",
    placeItems: "center",
    gap: "10px",
    padding: "40px",
    border: "1px solid black",
    borderRadius: "8px",
    backgroundColor: "lavender",
};
component.displayName = "Login";
export default component;