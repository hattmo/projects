import { IEntity } from "@hattmo/coreengine";
import badGuy from "../composites/badGuy";
import platform from "../composites/platform";
import playerInput from "../entities/playerInput";
import player from "../composites/player";

const level = (): IEntity<{}> => {
    return [{}, ({ create, die }) => {
        create(badGuy(300, 50, 10, 20));
        create(platform(250, 250, 300, 20));
        create(platform(150, 350, 300, 20));
        create(platform(145, 250, 20, 200));
        create(playerInput());
        create(player(330, 50, 10, 20))
        die();
        return {};
    }]
}

export default level;
