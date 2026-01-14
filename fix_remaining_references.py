#!/usr/bin/env python3
"""
Fix remaining 'crate::scenes::' references in files that were moved to screens directory.
"""

import re
from pathlib import Path

def fix_file(filepath):
    """Replace crate::scenes:: with crate::screens:: in a file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Replace crate::scenes:: with crate::screens::
        new_content = content.replace("crate::scenes::", "crate::screens::")
        
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
    
    # Process all .rs files in the screens directory
    screens_dir = base_dir / "core" / "src" / "screens"
    if screens_dir.exists():
        rs_files = list(screens_dir.rglob("*.rs"))
        count = 0
        for rs_file in rs_files:
            if fix_file(str(rs_file)):
                print(f"Fixed: {rs_file.relative_to(base_dir)}")
                count += 1
        print(f"\nFixed {count} files")
    else:
        print(f"Directory not found: {screens_dir}")

if __name__ == "__main__":
    main()
