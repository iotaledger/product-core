const path = require('path');
const CopyWebPlugin = require('copy-webpack-plugin');
const serverConfig = {
    target: 'node',
    entry: './examples/src/node.js',
    devtool: "source-map",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'node.js',
    },
    externals: [
        function ({ context, request }, callback) {
            if (/^@iota\/identity-wasm$/.test(request)) {
                // Externalize to a commonjs module
                return callback(null, 'commonjs ' + path.resolve(__dirname, '../node/identity_wasm.js'));
            }

            // Continue without externalizing the import
            callback();
        },
    ],
};

const serverTestConfig = {
    target: 'node',
    entry: './examples/src/test.js',
    devtool: "source-map",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'test.js',
    },
    externals: [
        function ({ context, request }, callback) {
            if (/^@iota\/identity-wasm$/.test(request)) {
                // Externalize to a commonjs module
                return callback(null, 'commonjs ' + path.resolve(__dirname, '../node/identity_wasm.js'));
            }

            // Continue without externalizing the import
            callback();
        },
    ],
};

const clientConfig = {
    target: 'web',
    entry: './examples/src/web.js',
    devtool: "source-map",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'web.js',
        library: {
            type: 'module',
        },
    },
    experiments: {
        topLevelAwait: true,
        outputModule: true,
    },
    resolve: {
        alias: {
            '@iota/identity-wasm': path.resolve(__dirname, '../web/identity_wasm.js'),
        },
    },
    plugins: [
        new CopyWebPlugin({
            patterns: [
                {
                    from: path.resolve(__dirname, "./src/index.html"),
                    to: path.resolve(__dirname, "dist"),
                },

                {
                    from: path.resolve(__dirname, "../web/identity_wasm_bg.wasm"),
                    to: path.resolve(__dirname, "dist"),
                }
            ]

        }),
    ],
};

module.exports = [serverConfig, serverTestConfig, clientConfig];
