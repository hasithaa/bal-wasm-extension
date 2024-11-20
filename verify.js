const fs = require('fs');
const path = require('path');

function verifyBuild() {
    const distPath = path.join(__dirname, 'dist');
    const files = ['bal_wasm_parser.js', 'bal_wasm_parser_bg.wasm', 'extension.js'];
    
    console.log('Verifying build in:', distPath);
    
    if (!fs.existsSync(distPath)) {
        console.error('dist directory not found!');
        process.exit(1);
    }

    const existingFiles = fs.readdirSync(distPath);
    console.log('Files in dist:', existingFiles);

    for (const file of files) {
        const filePath = path.join(distPath, file);
        if (!fs.existsSync(filePath)) {
            console.error(`Missing required file: ${file}`);
            process.exit(1);
        }
        const stats = fs.statSync(filePath);
        console.log(`${file}: ${stats.size} bytes`);
    }

    console.log('Build verification successful!');
}

verifyBuild();
