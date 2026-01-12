const fs = require('fs');
const path = require('path');
const { createCanvas, loadImage } = require('canvas');

async function convertSvgToPng(svgPath, pngPath, width = 512, height = 512) {
    try {
        // Read SVG file
        const svgBuffer = fs.readFileSync(svgPath);
        const svgString = svgBuffer.toString('utf-8');
        
        // Create canvas
        const canvas = createCanvas(width, height);
        const ctx = canvas.getContext('2d');
        
        // For SVG, we need to use a different approach
        // Convert SVG to data URL and load as image
        const img = await loadImage(`data:image/svg+xml;base64,${Buffer.from(svgString).toString('base64')}`);
        
        ctx.drawImage(img, 0, 0, width, height);
        
        // Save as PNG
        const out = fs.createWriteStream(pngPath);
        const stream = canvas.createPNGStream();
        stream.pipe(out);
        
        await new Promise((resolve, reject) => {
            out.on('finish', resolve);
            out.on('error', reject);
        });
        
        console.log(`✓ Converted: ${path.basename(svgPath)} -> ${path.basename(pngPath)}`);
    } catch (error) {
        console.error(`✗ Error: ${error.message}`);
    }
}

// Convert clang-logo.svg
const svgFile = 'studio-shell/resources/clang-logo.svg';
const pngFile = 'studio-shell/resources/clang-logo.png';

convertSvgToPng(svgFile, pngFile);
