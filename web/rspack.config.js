import { HtmlRspackPlugin } from "@rspack/core";
import path from "node:path";
import url from "url";

const __dirname = url.fileURLToPath(new URL(".", import.meta.url));

/** @type {import('@rspack/core').Configuration} */
export default {
    experiments: {
        css: true,
    },
    entry: {
        main: "./src/index.js",
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bundle.js",
        filename: "[name].[contenthash].js",
        clean: true,
    },
    module: {
        rules: [
            {
                test: /\.js$/,
                use: [
                    {
                        loader: "builtin:swc-loader",
                        options: {
                            jsc: {
                                parser: { syntax: "ecmascript" },
                            },
                        },
                    },
                ],
            },
            {
                test: /\.css$/,
                type: "css/auto",
                generator: {
                    localIdentName: "[name]__[local]",
                },
            },
        ],
        parser: {
            "css/auto": {
                namedExports: false,
            },
        },
    },
    plugins: [
        new HtmlRspackPlugin({
            template: "./public/index.html",
        }),
    ],
    devServer: {
        port: 3000,
        open: false,
        hot: true,
    },
};
