import * as webpack from "webpack";

const config: webpack.Configuration = {
  mode: "production",
  entry: {
    xss: "./src/xss.ts",
    loader: "./src/loader.ts",
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
    roots: [__dirname + "/src"],
  },
  output: {
    clean: true,
    path: __dirname + "/dst",
    filename: "[name].js",
  },
};

export default config;
