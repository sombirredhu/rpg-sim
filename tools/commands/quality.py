#!/usr/bin/env python3
"""
Quality assurance tools.
Run clippy, format check, test, asset validation.
"""

import subprocess
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent.parent

def run_cargo_check():
    """Run cargo check."""
    print("🔍 Running cargo check...")
    result = subprocess.run(
        'cargo check --all-targets',
        shell=True,
        cwd=PROJECT_ROOT
    )
    return result.returncode == 0

def run_clippy():
    """Run clippy linter."""
    print("📎 Running clippy...")
    result = subprocess.run(
        'cargo clippy --all-targets -- -D warnings',
        shell=True,
        cwd=PROJECT_ROOT
    )
    return result.returncode == 0

def run_fmt_check():
    """Check code formatting."""
    print("✨ Checking code format...")
    result = subprocess.run(
        'cargo fmt -- --check',
        shell=True,
        cwd=PROJECT_ROOT
    )
    return result.returncode == 0

def run_fmt_fix():
    """Auto-format code."""
    print("✨ Formatting code...")
    result = subprocess.run(
        'cargo fmt',
        shell=True,
        cwd=PROJECT_ROOT
    )
    return result.returncode == 0

def run_tests():
    """Run tests (if any)."""
    print("🧪 Running tests...")
    result = subprocess.run(
        'cargo test',
        shell=True,
        cwd=PROJECT_ROOT
    )
    return result.returncode == 0

def check_assets():
    """Run asset validation."""
    print("🎨 Checking assets...")
    # Import assets tool
    import sys
    sys.path.insert(0, str(PROJECT_ROOT / 'tools'))
    from commands.assets import check_assets
    return check_assets()

def run_all_checks(fix=False):
    """Run all quality checks."""
    print("🔬 Running full quality check...\n")

    checks = [
        ("Cargo check", run_cargo_check),
        ("Clippy", run_clippy),
        ("Format check", run_fmt_check if not fix else run_fmt_fix),
        ("Assets", check_assets),
        # Tests commented since project has none
        # ("Tests", run_tests),
    ]

    results = []
    for name, check_func in checks:
        print(f"\n{'='*60}")
        print(f"Checking: {name}")
        print('='*60)
        try:
            passed = check_func()
            results.append((name, passed))
            status = "✅ PASSED" if passed else "❌ FAILED"
            print(f"\n{status}")
        except Exception as e:
            print(f"❌ ERROR: {e}")
            results.append((name, False))

    print("\n" + "="*60)
    print("SUMMARY")
    print("="*60)

    all_passed = True
    for name, passed in results:
        status = "✅" if passed else "❌"
        print(f"{status} {name}")
        if not passed:
            all_passed = False

    if fix and not all_passed:
        print("\n💡 Some issues were auto-fixed. Re-run to verify.")

    return 0 if all_passed else 1

def run(args):
    """Run quality command."""
    if args.check == 'all':
        return run_all_checks(fix=args.fix)
    elif args.check == 'clippy':
        return 0 if run_clippy() else 1
    elif args.check == 'format':
        if args.fix:
            return 0 if run_fmt_fix() else 1
        else:
            return 0 if run_fmt_check() else 1
    elif args.check == 'tests':
        return 0 if run_tests() else 1
    elif args.check == 'assets':
        return 0 if check_assets() else 1

    return 0

if __name__ == "__main__":
    print("Quality module - use via 'python -m tools quality <check>'")
