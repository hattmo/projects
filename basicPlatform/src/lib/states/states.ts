export interface IBasicEntityState {
    name: string;
}
export interface IPhysicsState {
    x: number;
    y: number;
    w: number;
    h: number;
    dx: number;
    dy: number;
}

export interface ICollisionState {
    collideTop: boolean;
    collideBottom: boolean;
    collideLeft: boolean;
    collideRight: boolean;
}


export type PlayerActions = "stop" | "moveright" | "moveleft" | "jump";

export interface IPlayerControllableState {
    currentAction: PlayerActions,
}