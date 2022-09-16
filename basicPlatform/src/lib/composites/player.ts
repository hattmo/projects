import basicEntity from "../entities/basicEntity"
import rigidBody from "../modifiers/rigidBody"
import playerControlable from "../modifiers/playerControlable"
import { IBasicEntityState, IPhysicsState, ICollisionState, IPlayerControllableState } from "../states/states"

type IPlayerState = IBasicEntityState & IPhysicsState & ICollisionState & IPlayerControllableState
export const isPlayer = (target: any): target is IPlayerState => target?.name === "player"
export default (x, y, w, h) => {
    return playerControlable(
        rigidBody(0.1, 1)(
            basicEntity("player", x, y, w, h, 0, 0)
        )
    )
}