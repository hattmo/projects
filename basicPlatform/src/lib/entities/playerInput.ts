import { IEntity } from "@hattmo/coreengine";
import { IInputMessage } from "../messages/messages";
import { PlayerActions } from "../states/states";

export default (): IEntity<{
    lastAction: PlayerActions,
}> => {
    let currentAction: PlayerActions = "stop";
    window.addEventListener("keydown", (evnt) => {
        switch (evnt.key.toLowerCase()) {
            case "a":
                currentAction = "moveleft";
                break;
            case "d":
                currentAction = "moveright";
                break;
            case "w":
                currentAction = "jump";
                break;
        };
    }, false)
    window.addEventListener("keyup", (evnt) => {
        switch (evnt.key.toLowerCase()) {
            case "a":
                currentAction = currentAction === "moveleft" ? "stop" : currentAction;
                break;
            case "d":
                currentAction = currentAction === "moveright" ? "stop" : currentAction;
                break;
            case "w":
                currentAction = currentAction === "jump" ? "stop" : currentAction;
                break;
        }
    })
    return [{
        lastAction: currentAction,
    }, ({ state, send }) => {
        if (state.lastAction !== currentAction) {
            const inputMessage: IInputMessage = {
                action: currentAction,
                name: "inputMessage",
            };
            send(inputMessage);
        }
        return {
            lastAction: state.lastAction !== currentAction ? currentAction : state.lastAction,
        }
    }];
}