import React from "react";
import ReactDom from "react-dom";
import App from "../lib/App";

const root = document.createElement("div");
root.style.height = "100%";
document.body.appendChild(root);

if (!globalThis.api) {
  globalThis.api = {
    parseCSV: () =>
      Promise.resolve({
        headers: ["one", "two", "three", "four"],
        data: [{ one: "foo", two: "bar", three: "baz", four: "blooz" }],
      }),
    getFields: () => Promise.resolve(["foo", "bar", "baz"]),
  };
}

ReactDom.render(<App />, root);
