import * as webpack from "webpack";

const config: webpack.Configuration = {
  mode: "production",
  entry: {
    xss: "./src/xss.ts",
    loader: "./src/loader.ts",
  },
  output: {
    clean: true,
    path: __dirname + "/dst",
    filename: "[name].js",
  },
};

export default config;
