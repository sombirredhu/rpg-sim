#!/usr/bin/env python3
"""
Claude Code Development Tools for Realm of Bounties
A collection of utilities for building, testing, and maintaining the Bevy game.
"""

import sys
from pathlib import Path

# Add tools directory to path
TOOLS_DIR = Path(__file__).parent
sys.path.insert(0, str(TOOLS_DIR))

from commands import cli

if __name__ == "__main__":
    cli()
