import path from 'path';
import webpack from "webpack";
import WebpackDevServer from 'webpack-dev-server';

const ts = {
  test: /\.ts(x)?$/,
  use: 'ts-loader',
};
const fileLoad = {
  test: /\.(html|ico|png)$/,
  loader: 'file-loader',
  options: {
    name: '/[name].[ext]',
  }
}

const css = {
  test: /\.css$/,
  use: ['style-loader', 'css-loader'],
}

const devServer: WebpackDevServer.Configuration = {
  port: 8080,
  historyApiFallback: true,
  disableHostCheck: true,
  contentBase: path.resolve(__dirname, 'dist/'),
  inline: true,
  hot: true,
  proxy: {
    "/login": {
      changeOrigin: true,
      cookieDomainRewrite: "localhost",
      target: "http://localhost:8081",
    },
    "/auth": {
      changeOrigin: true,
      cookieDomainRewrite: "localhost",
      target: "http://localhost:8081",
    },
    "/api": {
      changeOrigin: true,
      cookieDomainRewrite: "localhost",
      target: "http://localhost:8082",
    }
  }
}

const config: webpack.Configuration = {
  entry: './src/main.tsx',
  output: {
    path: path.resolve(__dirname, 'dist/'),
    filename: 'bundle.js',
  },
  module: {
    rules: [ts, fileLoad, css],
  },
  resolve: {
    extensions: [' ', '.js', '.jsx', '.ts', '.tsx'],
  },

  devServer,
};



module.exports = config;
