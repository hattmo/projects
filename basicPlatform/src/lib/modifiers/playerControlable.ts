import { IEntity, IterateFunction } from "@hattmo/coreengine";
import { isInputMessage } from "../messages/messages";
import { IPlayerControllableState, IPhysicsState } from "../states/states";

export default <T>(target: IEntity<T & IPhysicsState>): IEntity<T & IPhysicsState & IPlayerControllableState> => {
    const [targetInitialState, targetInitialIterate] = target;
    const newIterate: IterateFunction<T & IPhysicsState & IPlayerControllableState> = (args) => {
        const { messages, state } = args;
        const targetState = targetInitialIterate(args);
        let newAction = state.currentAction;
        messages.filter(isInputMessage).forEach((mess) => {
            newAction = state.currentAction !== mess.action ? mess.action : state.currentAction;
        })
        const newState = { ...targetState, currentAction: newAction }
        switch (newAction) {
            case "stop":
                newState.dx = 0;
                break;
            case "moveleft":
                newState.dx = -1;
                break;
            case "moveright":
                newState.dx = 1;
                break;
            case "jump":
                newState.dy = -1;
        }
        return newState;
    }
    return [{ ...targetInitialState, currentAction: "stop" }, newIterate];
}