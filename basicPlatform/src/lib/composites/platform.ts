
import { IBasicEntityState, IPhysicsState } from "../states/states"

import basicEntity from "../entities/basicEntity";
import collideable from "../modifiers/collideable";
import { IEntity } from "@hattmo/coreengine";

type IPlatformState = IBasicEntityState & IPhysicsState;

export const isPlatform = (target: any): target is IPlatformState => target?.name === "platform"
export default (x, y, w, h, dx = 0, dy = 0): IEntity<IPlatformState> => {
    return collideable(
        basicEntity("platform", x, y, w, h, dx, dy)
    );
}