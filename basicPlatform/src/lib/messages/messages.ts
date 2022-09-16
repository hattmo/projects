import { PlayerActions } from "../states/states";

export interface BaseMessage {
    name: string,
}

export interface ICollideableMessage extends BaseMessage {
    name: "collideableMessage",
    x: number,
    y: number,
    w: number,
    h: number,
    dx: number,
    dy: number
}

export const isCollideableMessage = (target: any): target is ICollideableMessage => target?.name === "collideableMessage"

export interface IInputMessage extends BaseMessage {
    name: "inputMessage",
    action: PlayerActions,
}

export const isInputMessage = (target: any): target is IInputMessage => target?.name === "inputMessage";