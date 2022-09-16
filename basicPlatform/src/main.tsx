import coreEngine from "@hattmo/coreengine";
import renderer from "./lib/renderers/renderer";
import level from "./lib/levels/level";


const docstyle = document.body.style;
docstyle.margin = "0";
docstyle.overflow = "hidden";
docstyle.display = "grid";
docstyle.placeContent = "center";
docstyle.height = "100vh";
const root = document.createElement("canvas");
if (window.innerHeight > window.innerWidth) {
  root.width = window.innerWidth;
  root.height = window.innerWidth;
} else {
  root.width = window.innerHeight;
  root.height = window.innerHeight;
}
let timeHandle;
window.onresize = () => {
  clearTimeout(timeHandle);
  timeHandle = setTimeout(() => {
    if (window.innerHeight > window.innerWidth) {
      root.width = window.innerWidth;
      root.height = window.innerWidth;
    } else {
      root.width = window.innerHeight;
      root.height = window.innerHeight;
    }
  }, 100);
};

document.body.appendChild(root);
coreEngine(level(), [renderer(root)]);
