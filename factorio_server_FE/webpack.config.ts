import * as path from "path";
import * as webpack from "webpack";
import HtmlWebpackPlugin from "html-webpack-plugin";
// in case you run into any typescript error when configuring `devServer`
import "webpack-dev-server";
import { Configuration as devServerConfiguration } from "webpack-dev-server";

const ts: webpack.RuleSetRule = {
  test: /\.(ts|tsx)/,
  use: "ts-loader",
};
const devServer: devServerConfiguration = {
  hot: true,
  compress: true,
  client: {
    progress: true,
  },
  static: {
    directory: path.resolve(__dirname, "static"),
  },
};

const HtmlOptions: HtmlWebpackPlugin.Options = {
  minify: true,
};

const config: webpack.Configuration = {
  mode: "production",
  entry: "./src/bin/main.tsx",
  output: {
    path: path.resolve(__dirname, "static"),
    filename: "bundle.js",
  },
  module: {
    rules: [ts],
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js"],
  },
  plugins: [new HtmlWebpackPlugin(HtmlOptions)],
  devServer,
};

export default config;
