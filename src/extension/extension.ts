import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import * as util from 'util';

let wasmModule: any;

// Create a custom require function that handles built-in modules
const customRequire = (moduleName: string) => {
    switch (moduleName) {
        case 'util':
            return util;
        case 'path':
            return path;
        case 'fs':
            return fs;
        default:
            return require(moduleName);
    }
};

export async function activate(context: vscode.ExtensionContext) {
    try {
        console.log('Extension Path:', context.extensionPath);
        console.log('__dirname:', __dirname);

        const wasmPath = path.join(__dirname, 'bal_wasm_parser.js');
        console.log('Loading WASM from:', wasmPath);

        if (!fs.existsSync(wasmPath)) {
            throw new Error(`WASM module not found at: ${wasmPath}`);
        }

        // Read and evaluate the WASM module
        const wasmContent = fs.readFileSync(wasmPath, 'utf8');
        console.log('WASM file contents length:', wasmContent.length);

        // Create a wrapper that provides Node.js globals
        const wrapper = `
            (function(module, exports, require, __dirname, __filename) {
                ${wasmContent}
            })
        `;

        // Create a new module and evaluate it with our custom require and globals
        const moduleFunction = eval(wrapper);
        const moduleObj = { exports: {} };
        moduleFunction(
            moduleObj, 
            moduleObj.exports, 
            customRequire,
            path.dirname(wasmPath),
            wasmPath
        );

        wasmModule = moduleObj.exports;
        console.log('Available exports:', Object.keys(wasmModule));

        // Initialize the module
        if (typeof wasmModule.default === 'function') {
            await wasmModule.default();
        } else if (typeof wasmModule.init === 'function') {
            await wasmModule.init();
        }

        // Register providers
        context.subscriptions.push(
            vscode.languages.registerCompletionItemProvider(
                { scheme: 'file', language: 'ballerina' },
                new BallerinaCompletionProvider(),
                '.', ':', ' '
            )
        );

        vscode.window.showInformationMessage('Ballerina WASM Extension activated!');
    } catch (error) {
        console.error('Failed to activate extension:', error);
        vscode.window.showErrorMessage(`Failed to activate extension: ${error}`);
        throw error;
    }
}

class BallerinaCompletionProvider implements vscode.CompletionItemProvider {
    async provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position
    ): Promise<vscode.CompletionItem[]> {
        try {
            if (!wasmModule || !wasmModule.BallerinaParser) {
                console.log('WASM module or BallerinaParser not available');
                return [];
            }

            const lineText = document.lineAt(position.line).text;
            console.log('Current line text:', lineText);

            const parser = new wasmModule.BallerinaParser(document.getText());
            
            // Check if parse method exists
            if (typeof parser.parse === 'function') {
                await parser.parse();
            }

            // Basic suggestions
            const items: vscode.CompletionItem[] = [];

            // Keywords
            const keywords = ['function', 'service', 'resource', 'type', 'import', 'public', 'private'];
            keywords.forEach(keyword => {
                const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
                item.detail = `Ballerina ${keyword} keyword`;
                items.push(item);
            });

            console.log('Providing completions:', items.length);
            return items;

        } catch (error) {
            console.error('Error providing completions:', error);
            return [];
        }
    }
}

export function deactivate() {
    // Cleanup if needed
}
