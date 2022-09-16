import { IBasicEntityState, IPhysicsState } from "../states/states"
import rigidbody, { RigidBodyState } from "../modifiers/rigidBody"
import paceAI from "../modifiers/paceAI";
import basicEntity from "../entities/basicEntity";

type BadGuyState = IBasicEntityState & IPhysicsState & RigidBodyState;

export const isBadGuy = (target: any): target is BadGuyState => target?.name === "badGuy"
export default (x: number, y: number, w: number, h: number) => {
    return paceAI(-0.3)(
        rigidbody(0.1, 1)(
            basicEntity("badGuy", x, y, w, h)
        )
    )
}
