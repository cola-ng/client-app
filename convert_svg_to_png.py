from pathlib import Path
import base64
import io

def svg_to_png_browser(svg_path, png_path, width=512, height=512):
    """使用 Pillow 和 cairosvg 转换 SVG 到 PNG"""
    try:
        import cairosvg
        cairosvg.svg2png(
            url=str(svg_path),
            write_to=str(png_path),
            output_width=width,
            output_height=height
        )
        print(f"✓ Converted: {svg_path.name} -> {png_path.name}")
        return True
    except ImportError:
        print("cairosvg not installed, trying alternative method...")
        return False

def svg_to_png_pil(svg_path, png_path, width=512, height=512):
    """使用 PIL/Pillow 的替代方法"""
    try:
        from PIL import Image
        from io import BytesIO
        import subprocess
        
        # Try using rsvg-convert if available
        result = subprocess.run(
            ['rsvg-convert', '-w', str(width), '-h', str(height), str(svg_path)],
            capture_output=True,
            check=True
        )
        img = Image.open(BytesIO(result.stdout))
        img.save(png_path)
        print(f"✓ Converted: {svg_path.name} -> {png_path.name}")
        return True
    except (ImportError, FileNotFoundError, subprocess.CalledProcessError):
        print("Alternative conversion method failed")
        return False

if __name__ == "__main__":
    # Convert clang-logo.svg
    svg_file = Path("studio-shell/resources/clang-logo.svg")
    png_file = Path("studio-shell/resources/clang-logo.png")
    
    if svg_file.exists():
        success = svg_to_png_browser(svg_file, png_file)
        if not success:
            success = svg_to_png_pil(svg_file, png_file)
        
        if not success:
            print("\n⚠ No SVG converter available.")
            print("Please install one of:")
            print("  pip install cairosvg")
            print("  or install rsvg-convert")
            print("\nAlternatively, open the SVG in a browser and save as PNG,")
            print("or use an online converter like cloudconvert.com")
    else:
        print(f"✗ File not found: {svg_file}")
