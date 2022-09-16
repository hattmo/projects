import * as React from "react";
import * as ReactDom from "react-dom";
import "./css/main.css";
import Root from "./pages/root";
import * as OfflinePluginRuntime from "offline-plugin/runtime";

OfflinePluginRuntime.install();

const root = document.createElement("div");
root.className = "root";
root.id = "root";
document.body.appendChild(root);

ReactDom.render(
  (<Root />),
  root,
);
