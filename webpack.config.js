const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
    target: 'node',
    entry: './src/extension/extension.ts',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'extension.js',
        libraryTarget: 'commonjs2'
    },
    
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: 'ts-loader',
                exclude: /node_modules/
            }
        ]
    },

    resolve: {
        extensions: ['.ts', '.js']
    },

    plugins: [
        new CopyPlugin({
            patterns: [
                { 
                    from: 'src/rust/pkg/bal_wasm_parser.js',
                    to: 'bal_wasm_parser.js',
                    noErrorOnMissing: true
                },
                { 
                    from: 'src/rust/pkg/bal_wasm_parser_bg.wasm',
                    to: 'bal_wasm_parser_bg.wasm',
                    noErrorOnMissing: true
                }
            ],
        })
    ],

    externals: {
        vscode: 'commonjs vscode'
    },

    devtool: 'source-map',
    mode: 'development'
};
