# Ballerina WASM Extension

## Project Structure

```
bal-wasm-extension/
├── .vscode/                      # VSCode specific configs
│   └── launch.json              
├── src/
│   ├── extension/               # TypeScript VSCode extension code
│   │   ├── extension.ts         # Main extension entry point
│   │   ├── language-client.ts   # Custom language client implementation
│   │   └── wasm-bridge.ts       # Bridge between WASM and VSCode
│   │
│   └── rust/                    # Rust code that will compile to WASM
│       ├── src/
│       │   ├── lib.rs           # Main Rust library entry point
│       │   ├── parser/          # Ballerina parser implementation
│       │   │   ├── mod.rs
│       │   │   └── ast.rs       # AST definitions
│       │   ├── analyzer/        # Code analysis and validation
│       │   │   └── mod.rs
│       │   └── completion/      # Code completion logic
│       │       └── mod.rs
│       ├── Cargo.toml
│       └── build.rs            # Build script for WASM compilation
│
├── pkg/                        # Generated WASM package
├── dist/                      # Compiled extension
├── package.json              # Extension manifest
├── tsconfig.json            # TypeScript configuration
├── webpack.config.js        # Webpack configuration for bundling
└── README.md
```