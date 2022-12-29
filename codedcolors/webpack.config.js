const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const OfflinePlugin = require("offline-plugin");

const css = {
  test: /\.css$/,
  use: ["style-loader", "css-loader"],
};

const ts = {
  test: /\.ts(x)?$/,
  use: [
    {
      loader: "ts-loader",
      options: {
        compilerOptions: {
          jsx: "react",
        },
      },
    },
  ],
};

const assets = {
  test: /\.(png|jpe?g|gif|ico)$/,
  use: [
    {
      loader: "file-loader",
      options: {
        outputPath: "images",
        publicPath: "images",
        name: "[name].[ext]",
      },
    },
  ],
};

const video = {
  test: /\.webm$/,
  use: [
    {
      loader: "file-loader",
      options: {
        outputPath: "video",
        publicPath: "video",
        name: "[name].[ext]",
      },
    },
  ],
};

const config = {
  entry: {
    app: "./src/main.tsx",
  },
  output: {
    path: path.resolve(__dirname, "dst/"),
    filename: "js/[name]-[hash].js",
  },
  module: {
    rules: [css, ts, assets, video],
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: "Coded Colors",
      filename: "index.html",
      inject: "body",
      meta: {
        viewport: "width=device-width, initial-scale=1, shrink-to-fit=no",
      },
      minify: /staging/.test(process.env.NODE_ENV),
      chunks: ["app"],
    }),
    new OfflinePlugin(),
  ],
  resolve: {
    extensions: [" ", ".js", ".jsx", ".ts", ".tsx"],
  },
};
module.exports = config;
