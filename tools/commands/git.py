#!/usr/bin/env python3
"""
Git workflow helpers for feature development.
Automates branch creation, PR preparation, cleanup.
"""

import subprocess
import re
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent.parent

def run_git(cmd, capture=False):
    """Run git command."""
    try:
        if capture:
            result = subprocess.run(
                f"git {cmd}",
                shell=True,
                cwd=PROJECT_ROOT,
                capture_output=True,
                text=True
            )
            return result.returncode, result.stdout, result.stderr
        else:
            return subprocess.run(f"git {cmd}", shell=True, cwd=PROJECT_ROOT).returncode, None, None
    except Exception as e:
        return 1, "", str(e)

def create_feature_branch(name):
    """Create and switch to feature branch."""
    print(f"🌿 Creating feature branch: feature/{name}")

    # Check if already on a branch
    code, stdout, _ = run_git("branch --show-current", capture=True)
    current = stdout.strip()

    if current and current.startswith('feature/'):
        print(f"⚠️  Already on feature branch: {current}")
        confirm = input("Continue anyway? (y/N): ").lower()
        if confirm != 'y':
            return 1

    # Ensure main is up to date
    print("Updating main...")
    run_git("checkout main")
    run_git("pull origin main")

    # Create feature branch
    branch_name = f"feature/{name}"
    code, _, stderr = run_git(f"checkout -b {branch_name}")

    if code == 0:
        print(f"✅ Created and switched to {branch_name}")
        print("💡 Don't forget to commit your changes before creating PR")
        return 0
    else:
        print(f"❌ Failed to create branch: {stderr}")
        return 1

def prepare_review():
    """Prepare code for review."""
    print("🔍 Preparing for code review...")

    # Check current branch
    code, stdout, _ = run_git("branch --show-current", capture=True)
    branch = stdout.strip()

    if not branch or not branch.startswith('feature/'):
        print(f"⚠️  Not on a feature branch (current: {branch or 'detached'})")
        return 1

    # Check for uncommitted changes
    code, stdout, _ = run_git("status --porcelain", capture=True)
    if stdout.strip():
        print("❌ Uncommitted changes detected:")
        print(stdout)
        print("Please commit or stash before review")
        return 1

    # Show recent commits
    print("\n📜 Recent commits:")
    code, stdout, _ = run_git(f"log --oneline -10 origin/main..{branch}", capture=True)
    if stdout.strip():
        print(stdout)
    else:
        print("No commits ahead of origin/main")

    # Check branch diff stats
    print("\n📊 Diff statistics:")
    code, stdout, _ = run_git(f"diff --stat origin/main", capture=True)
    if stdout.strip():
        print(stdout)
    else:
        print("No differences from origin/main")

    print(f"\n✅ Ready for review on branch: {branch}")
    print("💡 Push with: git push -u origin {branch}")
    return 0

def cleanup_feature():
    """Clean up merged feature branches."""
    print("🧹 Cleaning up merged feature branches...")

    # Get all feature branches
    code, stdout, _ = run_git("branch -a", capture=True)
    branches = stdout.strip().split('\n')

    feature_branches = []
    for branch in branches:
        branch = branch.strip(' *')
        if branch.startswith('feature/'):
            feature_branches.append(branch)

    if not feature_branches:
        print("ℹ️  No feature branches found")
        return 0

    # Check which are merged
    merged_branches = []
    for branch in feature_branches:
        code, stdout, _ = run_git(f"branch --merged main", capture=True)
        if branch in stdout:
            merged_branches.append(branch)

    if not merged_branches:
        print("ℹ️  No merged feature branches to clean")
        return 0

    print(f"Found {len(merged_branches)} merged feature branches:")
    for b in merged_branches:
        print(f"  - {b}")

    confirm = input(f"\nDelete these {len(merged_branches)} branches? (y/N): ").lower()
    if confirm != 'y':
        print("Cancelled")
        return 0

    # Delete branches
    deleted = 0
    for branch in merged_branches:
        code, _, _ = run_git(f"branch -d {branch}")
        if code == 0:
            print(f"✅ Deleted {branch}")
            deleted += 1
        else:
            print(f"⚠️  Could not delete {branch}")

    print(f"\nDeleted {deleted}/{len(merged_branches)} branches")
    return 0

def git_status():
    """Show enhanced git status."""
    print("📋 Git Status")
    print("="*60)

    # Current branch
    code, stdout, _ = run_git("branch --show-current", capture=True)
    print(f"Branch: {stdout.strip() or '(detached)'}")

    # Status
    code, stdout, _ = run_git("status --short", capture=True)
    print("\nChanges:")
    if stdout.strip():
        print(stdout)
    else:
        print("  (clean)")

    # Ahead/behind
    code, stdout, _ = run_git("rev-list --left-right --count main...@{0}", capture=True)
    if stdout.strip():
        behind, ahead = stdout.strip().split()
        print(f"\nRelative to main: {ahead} ahead, {behind} behind")

    return 0

def run(args):
    """Run git command."""
    if args.action == 'feature':
        if not args.name:
            print("❌ Feature name required: --name <feature-name>")
            return 1
        return create_feature_branch(args.name)
    elif args.action == 'review':
        return prepare_review()
    elif args.action == 'cleanup':
        return cleanup_feature()
    elif args.action == 'status':
        return git_status()

    return 0

if __name__ == "__main__":
    print("Git module - use via 'python -m tools git <action>'")
