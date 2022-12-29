import { IEntity, IterateFunction } from "@hattmo/coreengine";
import { RigidBodyState } from "./rigidBody";

export default (walkSpeed: number) => {
    return <T>(target: IEntity<T & RigidBodyState>): IEntity<T & RigidBodyState> => {
        const [initialTargetState, initalTargetIterate] = target;
        const adjustedIterate: IterateFunction<T & RigidBodyState> = (args) => {
            const targetState = initalTargetIterate(args);
            let newDX = targetState.dx === 0 ? walkSpeed : targetState.dx;
            if (targetState.collideLeft) {
                newDX = -walkSpeed;
            } else if (targetState.collideRight) {
                newDX = walkSpeed;
            }
            return {
                ...targetState,
                dx: newDX,
            };
        };
        return [initialTargetState, adjustedIterate];
    }
}
