import { IEntity } from "@hattmo/coreengine";

import { IBasicEntityState, IPhysicsState } from "../states/states";

export default (
    name: string,
    x: number,
    y: number,
    w: number,
    h: number,
    dx: number = 0,
    dy: number = 0
): IEntity<IBasicEntityState & IPhysicsState> => {
    return [{ name, x, y, w, h, dx, dy }, ({ state }) => state];
}