import { useEffect } from "react";
import { ISetting } from "../../types";

type callback = (cal: ISetting[]) => void;

export default (cb: callback) => {
    useEffect(() => {
        fetch("/api/settings")
            .then((res) => {
                if (res.ok) {
                    return res.json();
                }
                throw new Error("Error requesting settings..");
            })
            .then((res) => {
                cb(res.settings);
            })
            .catch(() => { cb([]) })
    }, []);
};
