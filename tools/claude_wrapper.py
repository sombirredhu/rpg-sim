#!/usr/bin/env python3
"""
Claude Code skill wrapper for development tools.
Usage: python tools/claude_wrapper.py <action> [options]

This wrapper is designed to be called by Claude Code as a skill.
"""

import sys
import subprocess
from pathlib import Path

# Fix Windows console encoding for UTF-8
if sys.platform == 'win32' and hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    except Exception:
        pass

PROJECT_ROOT = Path(__file__).parent.parent

def run_tool(command):
    """Run a tool command and capture output."""
    full_cmd = f"python -m tools {command}"
    print(f"Running: {full_cmd}\n")

    result = subprocess.run(
        full_cmd,
        shell=True,
        cwd=PROJECT_ROOT,
        capture_output=True,
        text=True,
        encoding='utf-8',
        errors='ignore'
    )

    print(result.stdout)
    if result.stderr:
        print("STDERR:", result.stderr)

    return result.returncode

def main():
    if len(sys.argv) < 2:
        print(__doc__)
        print("\nAvailable actions:")
        print("  analyze    - Full code analysis")
        print("  build      - Build the project")
        print("  run        - Build and run game")
        print("  assets     - Check assets")
        print("  ecs        - ECS structure report")
        print("  quality    - Run all quality checks")
        print("  docs       - Generate documentation")
        print("  git:status - Git status")
        print("  git:feature <name> - Create feature branch")
        print("\nExample: python tools/claude_wrapper.py analyze")
        return 1

    action = sys.argv[1]

    # Map actions to tool commands
    if action == "analyze":
        return run_tool("analyze all")
    elif action == "build":
        return run_tool("build")
    elif action == "run":
        return run_tool("run")
    elif action == "assets":
        return run_tool("assets check")
    elif action == "ecs":
        return run_tool("ecs report")
    elif action == "quality":
        return run_tool("quality all")
    elif action == "docs":
        return run_tool("docs generate")
    elif action == "git:status":
        return run_tool("git status")
    elif action == "git:feature":
        if len(sys.argv) < 3:
            print("Error: feature name required")
            return 1
        return run_tool(f"git feature --name {sys.argv[2]}")
    else:
        print(f"Unknown action: {action}")
        return 1

if __name__ == "__main__":
    sys.exit(main())
