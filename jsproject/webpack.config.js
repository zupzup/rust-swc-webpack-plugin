const path = require('path');

module.exports = {
    name: 'swc-plugin-test',
    mode: "production",
    entry: path.join(__dirname, "src", "index.js"),
    output: {
        path: path.join(__dirname, "build"),
        filename: "bundle.js"
    },
    module: {
        rules: [
            {
                test: /\.js$/,
                use: [
                    {
                        loader: 'swc-loader',
                        options: {
                            jsc: {
                                parser: { syntax: 'ecmascript' },
                                experimental: {
                                    plugins: [["/home/zupzup/dev/oss/rust/rust-swc-webpack-plugin/target/wasm32-wasi/release/rust_swc_webpack_plugin.wasm", {}]]
                                },
                            },
                        },
                    }
                ],
                exclude: /node_modules/,
            },
        ],
    },
};
