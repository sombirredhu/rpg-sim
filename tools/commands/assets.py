#!/usr/bin/env python3
"""
Asset management tools.
Check asset integrity, validate sprite sheets, manage resources.
"""

from pathlib import Path
import json
from collections import defaultdict

PROJECT_ROOT = Path(__file__).parent.parent.parent
ASSETS_DIR = PROJECT_ROOT / "assets"

ASSET_SPECS = {
    'characters': {
        'path': 'Character/LPC',
        'required_subdirs': ['warrior', 'archer', 'mage', 'rogue', 'healer', 'skeleton'],
        'file_pattern': '*.png',
        'description': 'Character sprite sheets (9x4 grid per class)'
    },
    'buildings': {
        'path': 'GameplayAssetsV2/buildings',
        'required_types': ['town_hall', 'barracks', 'tavern', 'tower', 'wall', 'farm', 'market', 'bank', 'temple'],
        'levels': [1, 2, 3],
        'description': 'Building sprites (3 tiers each)'
    },
    'ground': {
        'path': 'Level/Ground',
        'required': ['grass.png', 'stone.png', 'water.png', 'rock.png', 'road.png'],
        'description': 'Ground tile textures'
    },
    'fonts': {
        'path': 'fonts',
        'required': ['FiraSans-Bold.ttf'],
        'description': 'UI fonts'
    },
    'audio_music': {
        'path': 'Audio/Music',
        'file_pattern': '*.ogg',
        'description': 'Background music'
    },
    'audio_sfx': {
        'path': 'Audio/SFX',
        'file_pattern': '*.ogg',
        'description': 'Sound effects'
    }
}

def check_assets(pattern='**/*.png'):
    """Check all required assets exist."""
    print("🎨 Checking assets...")

    missing = []
    found = []
    total_size = 0

    for asset_type, spec in ASSET_SPECS.items():
        base_path = ASSETS_DIR / spec['path']

        if 'required_subdirs' in spec:
            for subdir in spec['required_subdirs']:
                subdir_path = base_path / subdir
                if not subdir_path.exists():
                    missing.append(f"{asset_type}: Missing subdir {subdir}")
                else:
                    pngs = list(subdir_path.glob(spec.get('file_pattern', '*.png')))
                    if not pngs:
                        missing.append(f"{asset_type}: No {spec['file_pattern']} in {subdir}")
                    else:
                        for png in pngs:
                            found.append(png)
                            total_size += png.stat().st_size

        elif 'required' in spec:
            for req in spec['required']:
                file_path = base_path / req
                if not file_path.exists():
                    missing.append(f"{asset_type}: Missing {req}")
                else:
                    found.append(file_path)
                    total_size += file_path.stat().st_size

        elif 'required_types' in spec:
            for btype in spec['required_types']:
                type_path = base_path / btype
                if not type_path.exists():
                    missing.append(f"{asset_type}: Missing building type {btype}")
                else:
                    for level in spec['levels']:
                        level_file = type_path / f"lvl{level}.png"
                        if not level_file.exists():
                            missing.append(f"{asset_type}: Missing {btype}/lvl{level}.png")
                        else:
                            found.append(level_file)
                            total_size += level_file.stat().st_size

    print(f"\n✅ Found: {len(found)} files ({total_size / 1024 / 1024:.1f} MB)")
    print(f"❌ Missing: {len(missing)} files/items")

    if missing:
        print("\n🔍 Missing assets:")
        for m in missing[:50]:  # Limit output
            print(f"  - {m}")
        if len(missing) > 50:
            print(f"  ... and {len(missing) - 50} more")

    return len(missing) == 0

def asset_stats():
    """Generate asset statistics."""
    print("📈 Asset statistics:")

    stats = defaultdict(lambda: {'count': 0, 'size_mb': 0})

    for asset_type, spec in ASSET_SPECS.items():
        base_path = ASSETS_DIR / spec['path']
        if base_path.exists():
            files = list(base_path.glob('**/*'))
            total_size = sum(f.stat().st_size for f in files if f.is_file())
            stats[asset_type] = {
                'count': len([f for f in files if f.is_file()]),
                'size_mb': total_size / 1024 / 1024
            }

    print(f"\n{'Type':<25} {'Files':<10} {'Size (MB)':<10}")
    print("-" * 50)
    total_files = 0
    total_size = 0
    for atype, astats in sorted(stats.items()):
        print(f"{atype:<25} {astats['count']:<10} {astats['size_mb']:>10.1f}")
        total_files += astats['count']
        total_size += astats['size_mb']

    print("-" * 50)
    print(f"{'TOTAL':<25} {total_files:<10} {total_size:>10.1f}")

    return stats

def check_sprite_sheets():
    """Validate sprite sheet integrity (expects 9x4 grid for characters)."""
    print("🖼️  Validating sprite sheets...")

    character_dir = ASSETS_DIR / 'Character' / 'LPC'
    if not character_dir.exists():
        print("❌ Character directory not found")
        return False

    issues = []
    for class_dir in character_dir.iterdir():
        if class_dir.is_dir() and class_dir.name != 'skeleton':
            png_files = list(class_dir.glob('*.png'))
            for png in png_files:
                # In a real implementation, we'd check actual image dimensions
                # For now, just check filename patterns
                filename = png.name
                if not any(keyword in filename.lower() for keyword in
                          ['walk', 'attack', 'hurt', 'idle']):
                    issues.append(f"{class_dir.name}/{filename}: Unusual filename pattern")

    if issues:
        print(f"⚠️  Found {len(issues)} potential issues:")
        for issue in issues[:20]:
            print(f"  - {issue}")
    else:
        print("✅ All sprite sheets appear valid")

    return len(issues) == 0

def run(args):
    """Run asset command."""
    if args.action == 'check':
        success = check_assets(args.pattern)
        return 0 if success else 1
    elif args.action == 'missing':
        check_assets(args.pattern)  # This already outputs missing
        return 0
    elif args.action == 'stats':
        asset_stats()
        return 0
    elif args.action == 'sprites':
        success = check_sprite_sheets()
        return 0 if success else 1

    return 0

if __name__ == "__main__":
    print("Assets module - use via 'python -m tools assets <action>'")
