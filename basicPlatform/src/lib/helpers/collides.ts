import { IPhysicsState } from "../states/states"
type collisionDirection = "none" | "left" | "right" | "top" | "bottom";


export default (agent: IPhysicsState, target: IPhysicsState): collisionDirection => {
    if (!(
        agent.x <= target.x + target.w &&
        agent.x + agent.w >= target.x &&
        agent.y <= target.y + target.h &&
        agent.y + agent.h >= target.y
    )) {
        return "none";
    }
    const oldPos = {
        x: agent.x - agent.dx,
        y: agent.y - agent.dy,
        w: agent.w,
        h: agent.h,
    }
    if (oldPos.y + oldPos.h <= target.y) {
        return "top";
    } else if (oldPos.y >= target.y + target.h) {
        return "bottom";
    } else if (oldPos.x >= target.x + target.w) {
        return "right";
    } else if (oldPos.x + oldPos.w <= target.x) {
        return "left";
    }else {
        return "none";
    }
}
