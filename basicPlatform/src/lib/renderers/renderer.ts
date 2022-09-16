import { isPlatform } from "../composites/platform";
import { isBadGuy } from "../composites/badGuy";
import { isPlayer } from "../composites/player";


export default (root: HTMLCanvasElement) => {
    const ctx = root.getContext("2d");
    if (ctx !== null) {
        const renderer = (states: any[]) => {
            ctx.clearRect(0, 0, root.width, root.height);

            states.forEach((state) => {
                if (isPlatform(state)) {
                    const { x, y, w, h } = state;
                    ctx.fillStyle = "black";
                    ctx.fillRect(x, y, w, h);
                } else if (isBadGuy(state)) {
                    const { x, y, w, h } = state;
                    ctx.fillStyle = "red";
                    ctx.fillRect(x, y, w, h)
                } else if (isPlayer(state)) {
                    const { x, y, w, h } = state;
                    ctx.fillStyle = "green";
                    ctx.fillRect(x, y, w, h)
                }

            })
        }
        return renderer;
    } else {
        throw new Error("no render context");
    }
};
