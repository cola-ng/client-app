#!/usr/bin/env python3
"""
Fix screen_center back to scene_center and ScreenCenterScreen to SceneCenterScreen
"""

import re
from pathlib import Path

def fix_file(filepath):
    """Replace patterns in a file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        new_content = content
        # Replace ScreenCenterScreen with SceneCenterScreen
        new_content = new_content.replace("ScreenCenterScreen", "SceneCenterScreen")
        new_content = new_content.replace("ScreenCenterRef", "SceneCenterRef")
        new_content = new_content.replace("ScreenCenterWidgetRefExt", "SceneCenterWidgetRefExt")
        new_content = new_content.replace("ScreenCenter", "SceneCenter")
        # Replace screen_center with scene_center
        new_content = new_content.replace("screen_center_screen", "scene_center_screen")
        new_content = new_content.replace("screen_center", "scene_center")
        
        if content != new_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return True
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
    
    return False

def main():
    """Fix all remaining references."""
    base_dir = Path("D:\\Works\\colang\\desktop")
    
    # Process all .rs files
    print("Processing .rs files...")
    rs_files = list(base_dir.rglob("*.rs"))
    count = 0
    for rs_file in rs_files:
        if fix_file(str(rs_file)):
            print(f"Fixed: {rs_file.relative_to(base_dir)}")
            count += 1
    print(f"\nFixed {count} files")

if __name__ == "__main__":
    main()
