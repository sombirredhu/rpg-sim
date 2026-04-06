#!/usr/bin/env python3
"""
Entry point for: python -m tools <command>
"""

from .commands.cli import main

if __name__ == "__main__":
    import sys
    sys.exit(main())
