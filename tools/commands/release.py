#!/usr/bin/env python3
"""
Release management tools.
Version bumping, changelog generation, release preparation.
"""

import re
from datetime import datetime
from pathlib import Path
import subprocess

PROJECT_ROOT = Path(__file__).parent.parent.parent
CARGO_FILE = PROJECT_ROOT / "Cargo.toml"
CHANGELOG_FILE = PROJECT_ROOT / "CHANGELOG.md"

def get_current_version():
    """Extract current version from Cargo.toml."""
    content = CARGO_FILE.read_text(encoding='utf-8')
    match = re.search(r'version\s*=\s*"([^"]+)"', content)
    if match:
        return match.group(1)
    return None

def bump_version(current, bump_type='patch'):
    """Bump version according to semver."""
    major, minor, patch = map(int, current.split('.'))

    if bump_type == 'major':
        return f"{major + 1}.0.0"
    elif bump_type == 'minor':
        return f"{major}.{minor + 1}.0"
    else:  # patch
        return f"{major}.{minor}.{patch + 1}"

def update_cargo_version(new_version):
    """Update version in Cargo.toml."""
    content = CARGO_FILE.read_text(encoding='utf-8')
    new_content = re.sub(
        r'(version\s*=\s*")([^"]+)(")',
        rf'\1{new_version}\3',
        content
    )

    CARGO_FILE.write_text(new_content, encoding='utf-8')
    print(f"✅ Updated Cargo.toml to version {new_version}")

def generate_changelog_entry(version, commits):
    """Generate changelog entry for a version."""
    date = datetime.now().strftime('%Y-%m-%d')

    lines = [f"## [{version}] - {date}", ""]

    # Categorize commits
    categories = {
        'Added': [],
        'Changed': [],
        'Fixed': [],
        'Feat': [],
        'Fix': [],
    }

    for commit in commits:
        # Parse conventional commit format: type(scope): description
        match = re.match(r'^(\w+)(?:\(([^)]+)\))?:\s*(.+)', commit)
        if match:
            commit_type, scope, desc = match.groups()
            if commit_type in categories:
                entry = f"- {desc}"
                if scope:
                    entry = f"- **{scope}**: {desc}"
                categories[commit_type].append(entry)
            else:
                categories['Changed'].append(f"- {commit}")
        else:
            categories['Changed'].append(f"- {commit}")

    # Build changelog
    for cat, entries in categories.items():
        if entries:
            lines.append(f"### {cat}")
            lines.extend(entries)
            lines.append("")

    return '\n'.join(lines)

def update_changelog(version, new_entry):
    """Insert new entry at top of CHANGELOG.md."""
    if not CHANGELOG_FILE.exists():
        header = "# Changelog\n\nAll notable changes to this project will be documented in this file.\n\n"
        CHANGELOG_FILE.write_text(header, encoding='utf-8')

    content = CHANGELOG_FILE.read_text(encoding='utf-8')

    # Find position after title/intro
    if content.startswith('# Changelog'):
        parts = content.split('\n\n', 2)
        if len(parts) >= 3:
            header = parts[0] + '\n\n' + parts[1] + '\n\n'
            rest = parts[2]
        else:
            header = content + '\n\n'
            rest = ''
    else:
        header = ''
        rest = content

    new_content = header + new_entry + '\n' + rest
    CHANGELOG_FILE.write_text(new_content, encoding='utf-8')
    print(f"✅ Updated CHANGELOG.md with version {version}")

def get_git_commits_since_last_tag():
    """Get commit messages since last tag."""
    # Get latest tag
    code, stdout, _ = run_git("describe --tags --abbrev=0", capture=True)
    last_tag = stdout.strip() if code == 0 else None

    if last_tag:
        # Get commits since tag
        code, stdout, _ = run_git(f"log {last_tag}..HEAD --oneline", capture=True)
        commits = [line.split(maxsplit=1)[1] for line in stdout.strip().split('\n') if line.strip()]
    else:
        # No tags, get all commits on current branch
        code, stdout, _ = run_git("log --oneline", capture=True)
        commits = [line.split(maxsplit=1)[1] for line in stdout.strip().split('\n')[:50] if line.strip()]

    return commits

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

def prepare_release(version=None, bump_type='patch'):
    """Prepare a release: bump version, update changelog."""
    print("🚀 Preparing release...")

    current = get_current_version()
    if not current:
        print("❌ Could not determine current version")
        return 1

    if version:
        new_version = version
    else:
        new_version = bump_version(current, bump_type)

    print(f"  Current: {current}")
    print(f"  New: {new_version}")

    # Get commits for changelog
    print("  Gathering commits...")
    commits = get_git_commits_since_last_tag()

    if commits:
        entry = generate_changelog_entry(new_version, commits)
        update_changelog(new_version, entry)
    else:
        print("⚠️  No new commits found for changelog")

    # Update Cargo.toml
    update_cargo_version(new_version)

    print(f"\n✅ Release {new_version} prepared!")
    print("💡 Next steps:")
    print(f"  1. Review CHANGELOG.md")
    print(f"  2. Commit: git add Cargo.toml CHANGELOG.md && git commit -m 'chore: release {new_version}'")
    print(f"  3. Tag: git tag -a v{new_version} -m 'Version {new_version}'")
    print(f"  4. Push: git push && git push --tags")

    return 0

def show_changelog():
    """Display current changelog."""
    if CHANGELOG_FILE.exists():
        print(CHANGELOG_FILE.read_text(encoding='utf-8'))
    else:
        print("No CHANGELOG.md found")
    return 0

def run(args):
    """Run release command."""
    if args.action == 'changelog':
        return show_changelog()
    elif args.action == 'prepare':
        return prepare_release(args.version, 'patch')
    elif args.action == 'version':
        current = get_current_version()
        if current:
            print(f"Current version: {current}")
        return 0

    return 0

if __name__ == "__main__":
    print("Release module - use via 'python -m tools release <action>'")
