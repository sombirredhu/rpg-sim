#!/usr/bin/env python3
"""
Build and run commands for the Bevy game project.
"""

import subprocess
import sys
import time
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent.parent
CARGO_CMD = 'cargo'

def run_command(cmd, cwd=PROJECT_ROOT, capture=False):
    """Run a shell command and return result."""
    try:
        if capture:
            result = subprocess.run(
                cmd, shell=True, cwd=cwd,
                capture_output=True, text=True
            )
            return result.returncode, result.stdout, result.stderr
        else:
            return subprocess.run(cmd, shell=True, cwd=cwd).returncode, None, None
    except Exception as e:
        return 1, "", str(e)

def build_project(release=False, check_only=False):
    """Build the project."""
    print(f"{'🔍' if check_only else '🔨'} Building project...")

    if check_only:
        cmd = f"{CARGO_CMD} check"
    else:
        cmd = f"{CARGO_CMD} build {'--release' if release else ''}"

    print(f"  Command: {cmd}")
    code, stdout, stderr = run_command(cmd, capture=True)

    if code == 0:
        print("✅ Build successful!")
        if stdout:
            print(stdout[-500:])  # Last 500 chars
    else:
        print("❌ Build failed!")
        if stderr:
            print(stderr)
        return 1

    return 0

def run_game(release=False, auto_close=None):
    """Build and run the game."""
    print("🚀 Starting game...")

    cmd = f"{CARGO_CMD} run {'--release' if release else ''}"
    print(f"  Command: {cmd}")

    try:
        process = subprocess.Popen(
            cmd, shell=True, cwd=PROJECT_ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
            universal_newlines=True
        )

        start_time = time.time()

        # Monitor output
        for line in iter(process.stdout.readline, ''):
            print(line, end='')

            # Auto-close logic
            if auto_close:
                elapsed = time.time() - start_time
                if elapsed >= auto_close:
                    print(f"\n⏰ Auto-closing after {auto_close} seconds...")
                    process.terminate()
                    break

        process.wait()
        return process.returncode

    except KeyboardInterrupt:
        print("\n🛑 Game interrupted")
        return 130
    except Exception as e:
        print(f"❌ Error running game: {e}")
        return 1

def run(args):
    """Run build command."""
    return build_project(
        release=args.release,
        check_only=args.check_only
    )

def run_game_wrapper(args):
    """Run game command."""
    return run_game(
        release=args.release,
        auto_close=args.auto_close
    )

if __name__ == "__main__":
    # Simple test mode
    print("Build tools module - use via 'python -m tools build'")
