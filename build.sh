#!/bin/bash
set -e  # Exit on error

echo "Cleaning previous builds..."
rm -rf dist pkg node_modules

echo "Installing dependencies..."
npm install

echo "Building Rust WASM module..."
cd src/rust
wasm-pack build --target nodejs --out-dir ../../pkg
cd ../..

echo "Building extension..."
npx webpack --mode production

echo "Verifying build..."
if [ ! -f "dist/extension.js" ]; then
    echo "Error: extension.js is missing!"
    exit 1
fi

if [ ! -f "dist/bal_wasm_parser_bg.wasm" ]; then
    echo "Error: bal_wasm_parser_bg.wasm is missing!"
    exit 1
fi

echo "Files in dist directory:"
ls -la dist/

echo "Build completed successfully!"
