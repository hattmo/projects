import path from "path";
import { Configuration as DSConf } from "webpack-dev-server";
import HtmlWebpackPlugin from "html-webpack-plugin";
import { Configuration as WPConf, RuleSetRule } from "webpack";

const ts: RuleSetRule = {
  test: /\.ts(x)?$/,
  use: [
    {
      loader: "ts-loader",
    },
  ],
};

const css: RuleSetRule = {
  test: /\.css$/,
  use: [
    {
      loader: "style-loader",
    },
    {
      loader: "css-loader",
    },
  ],
};

const staticFiles: RuleSetRule = {
  test: /\.(html|ico|png)$/,
  use: [
    {
      loader: "file-loader",
      options: {
        name: "/[name].[ext]",
      },
    },
  ],
};

const devServer: DSConf = {
  port: 8081,
  historyApiFallback: true,
  disableHostCheck: true,
  contentBase: path.resolve(__dirname, "static/"),
  inline: true,
  hot: true,
};

const html = new HtmlWebpackPlugin({
  title: "PDF Gen",
  filename: "index.html",
  inject: "body",
  minify: false,
  meta: {
    viewport: "width=device-width, initial-scale=1, shrink-to-fit=no",
    ContentSecurity: {
      "http-equiv": "Content-Security-Policy",
      content: "default-src 'self' 'unsafe-eval' 'unsafe-inline'",
    },
  },
});

const config: WPConf = {
  entry: "./src/bin/main.tsx",
  output: {
    path: path.resolve(__dirname, "static/"),
    filename: "bundle.js",
  },
  module: {
    rules: [ts, css, staticFiles],
  },
  plugins: [html],
  devServer,
  resolve: {
    extensions: [" ", ".js", ".jsx", ".ts", ".tsx"],
  },
  devtool: "source-map",
};
module.exports = config;
