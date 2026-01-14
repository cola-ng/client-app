#!/usr/bin/env python3
"""
Refactor script to rename 'scenes' to 'screens' and 'Scene' to 'Screen' in struct names.
"""

import os
import re
import shutil
from pathlib import Path

# Define the mapping of old names to new names
REPLACEMENTS = {
    # Struct names
    "DialogScene": "DialogScreen",
    "HomeScene": "HomeScreen",
    "ReviewScene": "ReviewScreen",
    "SettingsScene": "SettingsScreen",
    "SceneCenter": "ScreenCenter",
    
    # Ref types
    "DialogSceneRef": "DialogScreenRef",
    "HomeSceneRef": "HomeScreenRef",
    "ReviewSceneRef": "ReviewScreenRef",
    "SettingsSceneRef": "SettingsScreenRef",
    "SceneCenterRef": "ScreenCenterRef",
    
    # WidgetRefExt traits
    "DialogSceneWidgetRefExt": "DialogScreenWidgetRefExt",
    "HomeSceneWidgetRefExt": "HomeScreenWidgetRefExt",
    "ReviewSceneWidgetRefExt": "ReviewScreenWidgetRefExt",
    "SettingsSceneWidgetRefExt": "SettingsScreenWidgetRefExt",
    "SceneCenterWidgetRefExt": "ScreenCenterWidgetRefExt",
    
    # Module paths (case-sensitive, exact matches)
    "scene_center": "screen_center",
    "scenes/": "screens/",
    "scenes::": "screens::",
}

# Patterns that are case-sensitive word-only replacements
WORD_REPLACEMENTS = {
    "DialogScene": "DialogScreen",
    "HomeScene": "HomeScreen",
    "ReviewScene": "ReviewScreen",
    "SettingsScene": "SettingsScreen",
    "SceneCenter": "ScreenCenter",
    "DialogSceneRef": "DialogScreenRef",
    "HomeSceneRef": "HomeScreenRef",
    "ReviewSceneRef": "ReviewScreenRef",
    "SettingsSceneRef": "SettingsScreenRef",
    "SceneCenterRef": "ScreenCenterRef",
    "DialogSceneWidgetRefExt": "DialogScreenWidgetRefExt",
    "HomeSceneWidgetRefExt": "HomeScreenWidgetRefExt",
    "ReviewSceneWidgetRefExt": "ReviewScreenWidgetRefExt",
    "SettingsSceneWidgetRefExt": "SettingsScreenWidgetRefExt",
    "SceneCenterWidgetRefExt": "ScreenCenterWidgetRefExt",
}

SUBSTRING_REPLACEMENTS = {
    "scene_center": "screen_center",
    "scene_center_scene": "screen_center_screen",
}

def refactor_content(content):
    """Replace all patterns in file content."""
    result = content
    
    # Apply word replacements (whole word boundaries)
    for old, new in WORD_REPLACEMENTS.items():
        pattern = r'\b' + re.escape(old) + r'\b'
        result = re.sub(pattern, new, result)
    
    # Apply substring replacements
    for old, new in SUBSTRING_REPLACEMENTS.items():
        result = result.replace(old, new)
    
    return result

def process_file(filepath):
    """Process a single file to refactor content."""
    try:
        with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
        
        refactored = refactor_content(content)
        
        if content != refactored:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(refactored)
            return True
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
    
    return False

def rename_file(old_path, new_path):
    """Rename a file."""
    try:
        if os.path.exists(new_path):
            os.remove(new_path)
        os.rename(old_path, new_path)
        print(f"Renamed: {old_path} -> {new_path}")
        return True
    except Exception as e:
        print(f"Error renaming {old_path}: {e}")
        return False

def rename_directory(old_path, new_path):
    """Rename a directory."""
    try:
        if os.path.exists(new_path):
            shutil.rmtree(new_path)
        os.rename(old_path, new_path)
        print(f"Renamed directory: {old_path} -> {new_path}")
        return True
    except Exception as e:
        print(f"Error renaming directory {old_path}: {e}")
        return False

def main():
    """Main refactoring function."""
    base_dir = Path("D:\\Works\\colang\\desktop")
    
    # Step 1: Process all .rs files
    print("Step 1: Processing .rs files...")
    rs_files = list(base_dir.rglob("*.rs"))
    for rs_file in rs_files:
        if process_file(str(rs_file)):
            print(f"  Refactored: {rs_file.relative_to(base_dir)}")
    
    # Step 2: Rename scene_center directory to screen_center
    print("\nStep 2: Renaming directories...")
    scenes_dir = base_dir / "core" / "src" / "scenes"
    screens_dir = base_dir / "core" / "src" / "screens"
    scene_center_dir = scenes_dir / "scene_center"
    screen_center_dir = screens_dir / "screen_center"
    
    if scene_center_dir.exists():
        # First ensure screens directory exists
        if not screens_dir.exists():
            # Rename scenes to screens
            rename_directory(str(scenes_dir), str(screens_dir))
        else:
            # Move scene_center inside screens
            if scene_center_dir.exists():
                rename_directory(str(scene_center_dir), str(screen_center_dir))
    
    # Step 3: Rename files
    print("\nStep 3: Renaming files...")
    file_renames = {
        "scene_center/scene_center_scene.rs": "screen_center/screen_center_screen.rs",
        "home/home_scene.rs": "home/home_screen.rs",
        "dialog/dialog_scene.rs": "dialog/dialog_screen.rs",
        "review/review_scene.rs": "review/review_screen.rs",
        "settings/settings_scene.rs": "settings/settings_screen.rs",
        "scene_center.rs": "screen_center.rs",
        "home.rs": "home.rs",
        "dialog.rs": "dialog.rs",
        "review.rs": "review.rs",
        "settings.rs": "settings.rs",
        "scenes.rs": "screens.rs",
    }
    
    for old_rel, new_rel in file_renames.items():
        if "scene_center" in old_rel:
            old_path = base_dir / "core" / "src" / "screens" / old_rel
            new_path = base_dir / "core" / "src" / "screens" / new_rel
        else:
            old_path = base_dir / "core" / "src" / old_rel
            new_path = base_dir / "core" / "src" / new_rel
        
        if old_path.exists() and old_path != new_path:
            rename_file(str(old_path), str(new_path))
    
    print("\nRefactoring complete!")

if __name__ == "__main__":
    main()
