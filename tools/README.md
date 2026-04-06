# Development Tools

A comprehensive Python-based toolkit for developing "Realm of Bounties" (Bevy 0.6.1 game).

## Installation

No installation required! Just ensure you have Python 3.7+ and cargo (Rust) installed.

## Quick Start

```bash
# Make executable (optional)
chmod +x tools/*.py

# Or run as module
python -m tools <command> [options]

# Examples
python -m tools build
python -m tools run --auto-close 5
python -m tools analyze all
python -m tools quality check
```

## Available Commands

### Build & Run

```bash
python -m tools build [--release] [--check-only]
  🔨 Build the project

python -m tools run [--release] [--auto-close SECONDS]
  🚀 Build and run the game
```

### Analysis

```bash
python -m tools analyze all|structure|complexity|deps [--output FILE]
  📊 Code analysis
  - all: Comprehensive analysis
  - structure: File organization, functions, imports
  - complexity: Nesting depth, code metrics
  - deps: Module dependencies
```

### Assets

```bash
python -m tools assets check|missing|stats|sprites [--pattern '**/*.png']
  🎨 Asset management
  - check: Verify all required assets exist
  - missing: List missing assets
  - stats: Show asset statistics
  - sprites: Validate sprite sheets
```

### ECS Analysis

```bash
python -m tools ecs components|systems|resources|report [--output FILE]
  🔍 Analyze Bevy ECS structure
  - components: List all Component structs
  - systems: List all systems with queries
  - resources: List all Resources
  - report: Full ECS analysis with relationships
```

### Game Testing

```bash
python -m tools game test|validate|health|save-check [--branch NAME]
  🎮 Game testing
  - test: Quick build test
  - validate: Validate save files
  - health: Comprehensive health check
  - save-check: Verify save file format
```

### Documentation

```bash
python -m tools docs generate|components|api|gdd [--output DIR]
  📚 Documentation generation
  - generate: All docs
  - components: Component reference
  - api: API documentation
  - gdd: GDD summary
```

### Git Workflow

```bash
python -m tools git feature --name NAME
  🌿 Create feature branch

python -m tools git review
  🔍 Prepare for code review

python -m tools git cleanup
  🧹 Delete merged feature branches

python -m tools git status
  📋 Enhanced status
```

### Release Management

```bash
python -m tools release changelog|prepare|version [--version X.Y.Z]
  🚀 Release management
  - changelog: Display changelog
  - prepare: Bump version, update changelog
  - version: Show current version
```

### Quality Checks

```bash
python -m tools quality all|clippy|format|tests|assets [--fix]
  🔬 Quality assurance
  - all: Run all checks
  - clippy: Run linter
  - format: Check formatting (or --fix to auto-format)
  - tests: Run tests (none currently)
  - assets: Validate assets
```

## Examples

Full development workflow:

```bash
# 1. Create feature branch
python -m tools git feature --name escort-merchant-bounties

# 2. Make your code changes...

# 3. Check everything before committing
python -m tools quality all

# 4. Analyze code
python -m tools analyze all --output analysis.json

# 5. Check assets
python -m tools assets check

# 6. Verify ECS structure
python -m tools ecs report --output ecs-report.json

# 7. Prepare for review
python -m tools git review

# 8. Push and create PR
git push -u origin feature/escort-merchant-bounties
# Create PR on GitHub

# 9. After merge, cleanup
python -m tools git cleanup
```

## Output Formats

Most commands can output to JSON for tooling integration:

```bash
python -m tools analyze all --output analysis.json
python -m tools ecs report --output ecs-report.json
python -m tools docs generate --output docs/
```

## Requirements

- Python 3.7 or higher
- Rust/cargo (for build/run commands)
- Bevy project structure (src/*.rs, assets/, Cargo.toml)

## Architecture

Tools are organized as subcommands under `tools/commands/`:

```
tools/
├── __init__.py          # Entry point
├── cli.py               # Main CLI parser
├── commands/
│   ├── build.py         # Build/run commands
│   ├── analyze.py       # Code analysis
│   ├── assets.py        # Asset validation
│   ├── ecs.py           # ECS analysis
│   ├── game.py          # Game testing
│   ├── docs.py          # Docs generation
│   ├── git.py           # Git workflow
│   ├── release.py       # Release management
│   └── quality.py       # Quality checks
└── README.md
```

## Notes

- All tools respect the Bevy 0.6.1 API conventions
- Asset paths follow the project's GDD specifications
- ECS analysis understands Bevy's component/system/resource patterns
- No external Python dependencies - uses only standard library
