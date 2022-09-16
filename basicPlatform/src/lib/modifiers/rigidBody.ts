import { IEntity, IterateFunction } from "@hattmo/coreengine";
import { IPhysicsState, ICollisionState } from "../states/states";
import { isCollideableMessage } from "../messages/messages";
import collides from "../helpers/collides";

export type RigidBodyState = IPhysicsState & ICollisionState;

export default (gravity: number, maxFallSpeed: number) => {
    return <T>(target: IEntity<T & IPhysicsState>): IEntity<T & RigidBodyState> => {
        const [initialTargetState, initalTargetIterate] = target;
        const adjustedIterate: IterateFunction<T & RigidBodyState> = (args) => {
            const targetState = initalTargetIterate(args);
            const nextState: T & RigidBodyState = {
                ...targetState,
                collideBottom: false,
                collideLeft: false,
                collideRight: false,
                collideTop: false,
                x: targetState.x + targetState.dx,
                y: targetState.y + targetState.dy,
                dy: targetState.dy >= maxFallSpeed ? maxFallSpeed : targetState.dy + gravity,
            };

            const { messages } = args;
            messages.filter(isCollideableMessage).forEach((message) => {
                switch (collides(nextState, message)) {
                    case "top":
                        nextState.y = message.y - nextState.h;
                        nextState.dy = 0;
                        nextState.collideBottom = true;
                        break;
                    case "bottom":
                        nextState.y = message.y + message.h;
                        nextState.dy = 0;
                        nextState.collideTop = true;
                        break;
                    case "left":
                        nextState.x = message.x - nextState.w;
                        nextState.dx = 0;
                        nextState.collideRight = true;
                        break;
                    case "right":
                        nextState.x = message.x + message.w;
                        nextState.dx = 0;
                        nextState.collideLeft = true;
                        break;
                    default:
                        break;
                }
            });
            return nextState;
        };
        return [
            {
                ...initialTargetState,
                collideBottom: false,
                collideLeft: false,
                collideRight: false,
                collideTop: false,
            },
            adjustedIterate
        ];
    }
}
