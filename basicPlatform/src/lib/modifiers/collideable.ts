import { IEntity, IterateFunction } from "@hattmo/coreengine";
import { ICollideableMessage } from "../messages/messages";
import { IPhysicsState } from "../states/states";

export default <T>(target: IEntity<T & IPhysicsState>): IEntity<T & IPhysicsState> => {
    const [targetInitialState, targetInitialIterate] = target;
    const newIterate: IterateFunction<T & IPhysicsState> = (args) => {
        const { state, send } = args;
        const platformMessage: ICollideableMessage = {
            ...state,
            name: "collideableMessage",
        }
        send(platformMessage);
        const targetState = targetInitialIterate(args);
        return targetState;
    }
    return [targetInitialState, newIterate];
}