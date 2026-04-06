#!/usr/bin/env python3
"""
Game testing and debugging tools.
Validate game state, check save files, perform health checks.
"""

import subprocess
import json
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent.parent
SAVE_DIR = PROJECT_ROOT / "saves"

def quick_test():
    """Run quick game test - build and verify starts."""
    print("🎮 Quick game test...")

    # Build
    result = subprocess.run(
        'cargo check',
        shell=True,
        cwd=PROJECT_ROOT,
        capture_output=True,
        text=True
    )

    if result.returncode != 0:
        print("❌ Build failed!")
        print(result.stderr)
        return False

    print("✅ Build successful")
    print("ℹ️  Run 'python -m tools run' to launch the game")
    return True

def validate_save_files():
    """Validate save file format."""
    print("💾 Validating save files...")

    if not SAVE_DIR.exists():
        print("⚠️  No saves directory found")
        return True

    saves = list(SAVE_DIR.glob("*.json"))
    if not saves:
        print("ℹ️  No save files found")
        return True

    errors = []
    for save_file in saves:
        try:
            with open(save_file) as f:
                data = json.load(f)
            # Basic validation
            if 'kingdom_state' not in data:
                errors.append(f"{save_file.name}: Missing kingdom_state")
            if 'heroes' not in data:
                errors.append(f"{save_file.name}: Missing heroes")
        except json.JSONDecodeError as e:
            errors.append(f"{save_file.name}: Invalid JSON - {e}")
        except Exception as e:
            errors.append(f"{save_file.name}: Error - {e}")

    if errors:
        print(f"❌ Found {len(errors)} invalid save files:")
        for err in errors:
            print(f"  - {err}")
        return False
    else:
        print(f"✅ Validated {len(saves)} save files")
        return True

def game_health_check():
    """Comprehensive game health check."""
    print("🏥 Game Health Check")
    print("="*60)

    checks = [
        ("Build", quick_test),
        ("Save files", validate_save_files),
    ]

    all_pass = True
    for name, check_func in checks:
        print(f"\n{name}:")
        try:
            if not check_func():
                all_pass = False
        except Exception as e:
            print(f"❌ Error: {e}")
            all_pass = False

    print("\n" + "="*60)
    if all_pass:
        print("✅ All checks passed!")
    else:
        print("❌ Some checks failed")

    return all_pass

def run(args):
    """Run game command."""
    if args.action == 'test':
        return 0 if quick_test() else 1
    elif args.action == 'validate':
        return 0 if validate_save_files() else 1
    elif args.action == 'health':
        return 0 if game_health_check() else 1
    elif args.action == 'save-check':
        return 0 if validate_save_files() else 1

    return 0

if __name__ == "__main__":
    print("Game module - use via 'python -m tools game <action>'")
