#!/usr/bin/env python3
"""
Main CLI for all development tools.
Usage: python -m tools <command> [options]
"""

import argparse
import sys
from pathlib import Path

# Fix Windows console encoding for emojis
if sys.platform == 'win32' and hasattr(sys.stdout, 'reconfigure'):
    try:
        sys.stdout.reconfigure(encoding='utf-8', errors='ignore')
    except Exception:
        pass

# Import all command modules
from . import build, analyze, assets, ecs, game, docs, git, release, quality

def main():
    parser = argparse.ArgumentParser(
        description="Development tools for Realm of Bounties (Bevy 0.6.1 game)",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s build          # Build the project
  %(prog)s run            # Build and run the game
  %(prog)s analyze all    # Run complete code analysis
  %(prog)s assets check   # Verify all assets exist
  %(prog)s ecs report     # Generate ECS component report
  %(prog)s game test      # Quick gameplay test
  %(prog)s docs generate  # Generate documentation
  %(prog)s quality check  # Run all quality checks
        """
    )

    subparsers = parser.add_subparsers(dest='command', help='Available commands')

    # Build commands
    build_parser = subparsers.add_parser('build', help='Build the project')
    build_parser.add_argument('--release', action='store_true', help='Release build')
    build_parser.add_argument('--check-only', action='store_true', help='Type check only')
    build_parser.set_defaults(func=build.run)

    run_parser = subparsers.add_parser('run', help='Build and run the game')
    run_parser.add_argument('--release', action='store_true', help='Release mode')
    run_parser.add_argument('--auto-close', type=int, metavar='SECONDS',
                            help='Auto-close after N seconds')
    run_parser.set_defaults(func=build.run_game)

    # Analyze commands
    analyze_parser = subparsers.add_parser('analyze', help='Code analysis')
    analyze_parser.add_argument('type', choices=['all', 'structure', 'complexity', 'deps'],
                                help='Analysis type')
    analyze_parser.add_argument('--output', type=Path, help='Output file (JSON/HTML)')
    analyze_parser.set_defaults(func=analyze.run)

    # Asset commands
    assets_parser = subparsers.add_parser('assets', help='Asset management')
    assets_parser.add_argument('action', choices=['check', 'missing', 'stats', 'sprites'],
                               help='Asset action')
    assets_parser.add_argument('--pattern', default='**/*.png',
                               help='Glob pattern for assets')
    assets_parser.set_defaults(func=assets.run)

    # ECS commands
    ecs_parser = subparsers.add_parser('ecs', help='ECS analysis')
    ecs_parser.add_argument('report', choices=['components', 'systems', 'resources', 'report'],
                            help='ECS report type')
    ecs_parser.add_argument('--output', type=Path, help='Output file')
    ecs_parser.set_defaults(func=ecs.run)

    # Game commands
    game_parser = subparsers.add_parser('game', help='Game testing and debugging')
    game_parser.add_argument('action', choices=['test', 'validate', 'health', 'save-check'],
                             help='Game action')
    game_parser.add_argument('--branch', help='Git branch to test')
    game_parser.set_defaults(func=game.run)

    # Docs commands
    docs_parser = subparsers.add_parser('docs', help='Documentation')
    docs_parser.add_argument('action', choices=['generate', 'components', 'api', 'gdd'],
                             help='Docs action')
    docs_parser.add_argument('--output', type=Path, default=Path('docs'),
                            help='Output directory')
    docs_parser.set_defaults(func=docs.run)

    # Git commands
    git_parser = subparsers.add_parser('git', help='Git workflow helpers')
    git_parser.add_argument('action', choices=['feature', 'review', 'cleanup', 'status'],
                            help='Git action')
    git_parser.add_argument('--name', help='Feature name')
    git_parser.set_defaults(func=git.run)

    # Release commands
    release_parser = subparsers.add_parser('release', help='Release management')
    release_parser.add_argument('action', choices=['changelog', 'version', 'prepare'],
                                help='Release action')
    release_parser.add_argument('--version', help='Version number')
    release_parser.set_defaults(func=release.run)

    # Quality commands
    quality_parser = subparsers.add_parser('quality', help='Quality checks')
    quality_parser.add_argument('check', choices=['all', 'clippy', 'format', 'tests', 'assets'],
                                help='Quality check type')
    quality_parser.add_argument('--fix', action='store_true', help='Auto-fix issues')
    quality_parser.set_defaults(func=quality.run)

    # Parse and execute
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        return 1

    try:
        return args.func(args)
    except KeyboardInterrupt:
        print("\nInterrupted")
        return 130
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        if '--debug' in sys.argv:
            import traceback
            traceback.print_exc()
        return 1

if __name__ == "__main__":
    sys.exit(main())
