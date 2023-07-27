const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");

module.exports = {
  entry: "./index.ts",
  output: {
    path: path.resolve(__dirname, "./dist"),
    filename: "index.js",
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      // TODO(appcypher): Wasm inlining does not work yet because of an issue with webpack/wasm-bindgen
      // See: https://rustwasm.github.io/wasm-bindgen/examples/hello-world.html
      // {
      //   test: /\.wasm$/,
      //   type: "asset/inline",
      // },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin(),
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    new webpack.ProvidePlugin({
      TextDecoder: ["text-encoding", "TextDecoder"],
      TextEncoder: ["text-encoding", "TextEncoder"],
    }),
  ],
  resolve: {
    extensions: ['.ts', '.js'],
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
    topLevelAwait: true,
  },
};
