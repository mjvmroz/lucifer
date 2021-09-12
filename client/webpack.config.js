const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const appConfig = {
    entry: "./app/main.ts",
    devServer: { contentBase: dist },
    plugins: [
        new HtmlWebpackPlugin({
            template: "index.html",
            root: path.resolve(__dirname, "."),
        }),
        new MiniCssExtractPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "../compute"),
        }),
    ],
    module: {
        rules: [
            {test: /\.worker\.ts$/, loader: 'worker-loader'},
            {
                test: /\.tsx?$/,
                use: "ts-loader",
                exclude: /node_modules/,
            },
            {
                test: /\.css$/i,
                use: [
                    {
                        loader: MiniCssExtractPlugin.loader,
                        options: { publicPath: "css/" },
                    },
                    "css-loader",
                ],
            },
            {
                test: /\.(png|jpe?g|gif|svg|ico)$/i,
                use: [{ loader: "file-loader?name=./static/[name].[ext]" }],
            },
            {
                test: /\.(webmanifest|xml)$/i,
                use: [{ loader: "file-loader?name=./[name].[ext]" }],
            },
        ],
    },
    resolve: {
        extensions: [".ts", ".js"],
    },
    output: { path: dist, filename: "app.js" },
    experiments: { syncWebAssembly: true },
};

module.exports = [appConfig];
