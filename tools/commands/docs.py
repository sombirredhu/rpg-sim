#!/usr/bin/env python3
"""
Documentation generation tools.
Generate API docs, component reference, architecture diagrams, etc.
"""

import re
from pathlib import Path
from datetime import datetime

PROJECT_ROOT = Path(__file__).parent.parent.parent
SRC_DIR = PROJECT_ROOT / "src"
DOCS_DIR = PROJECT_ROOT / "docs"

def generate_component_docs():
    """Generate component reference documentation."""
    print("📚 Generating component reference...")

    # Simple parser for component definitions
    components = []

    for rust_file in SRC_DIR.glob("*.rs"):
        content = rust_file.read_text(encoding='utf-8')
        # Find component structs
        pattern = r'#\[derive\([^)]+Component[^)]*\)\s+(?:pub\s+)?struct\s+(\w+)'
        for match in re.finditer(pattern, content):
            comp_name = match.group(1)

            # Try to extract brief comment
            before = content[:match.start()]
            last_comment = before.rfind('///')
            if last_comment != -1:
                comment_line = before[last_comment:].split('\n')[0]
                comment = comment_line.replace('///', '').strip()
            else:
                comment = ""

            components.append({
                'name': comp_name,
                'file': rust_file.name,
                'description': comment
            })

    # Generate markdown
    DOCS_DIR.mkdir(exist_ok=True)
    output_file = DOCS_DIR / "components.md"

    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("# Component Reference\n\n")
        f.write(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}\n\n")
        f.write(f"Total components: {len(components)}\n\n")

        f.write("## By File\n\n")
        by_file = {}
        for comp in components:
            by_file.setdefault(comp['file'], []).append(comp)

        for filename, comps in sorted(by_file.items()):
            f.write(f"### {filename}\n\n")
            f.write("| Component | Description |\n")
            f.write("|-----------|-------------|\n")
            for comp in sorted(comps, key=lambda x: x['name']):
                desc = comp['description'] or '_No description_'
                f.write(f"| `{comp['name']}` | {desc} |\n")
            f.write("\n")

    print(f"✅ Component reference saved to {output_file}")
    return True

def generate_api_docs():
    """Generate API documentation from source."""
    print("📖 Generating API documentation...")

    DOCS_DIR.mkdir(exist_ok=True)
    output_file = DOCS_DIR / "api.md"

    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("# API Documentation\n\n")
        f.write(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}\n\n")

        for rust_file in sorted(SRC_DIR.glob("*.rs")):
            content = rust_file.read_text(encoding='utf-8')
            f.write(f"## {rust_file.name}\n\n")

            # Extract module doc comment
            mod_doc_match = re.search(r'//!([^\n]+)', content)
            if mod_doc_match:
                f.write(f"*{mod_doc_match.group(1).strip()}*\n\n")

            # List public structs
            structs = re.findall(r'pub\s+struct\s+(\w+)', content)
            if structs:
                f.write("### Structs\n\n")
                for struct in structs:
                    f.write(f"- `{struct}`\n")
                f.write("\n")

            # List public enums
            enums = re.findall(r'pub\s+enum\s+(\w+)', content)
            if enums:
                f.write("### Enums\n\n")
                for enum in enums:
                    f.write(f"- `{enum}`\n")
                f.write("\n")

            # List public functions
            funcs = re.findall(r'pub\s+fn\s+(\w+)', content)
            if funcs:
                f.write("### Functions\n\n")
                for func in funcs:
                    f.write(f"- `{func}`\n")
                f.write("\n")

    print(f"✅ API docs saved to {output_file}")
    return True

def generate_gdd_summary():
    """Generate summary from GDD."""
    print("📄 Generating GDD summary...")

    gdd_file = PROJECT_ROOT / "Realm_of_Bounties_GDD.txt"
    if not gdd_file.exists():
        print("⚠️  GDD file not found")
        return False

    output_file = DOCS_DIR / "gdd_summary.md"

    with open(gdd_file, encoding='utf-8') as f:
        content = f.read()

    # Extract sections
    sections = {}
    current_section = None
    current_content = []

    for line in content.split('\n'):
        if line.strip() and (line.startswith('# ') or line.startswith('## ')):
            if current_section:
                sections[current_section] = '\n'.join(current_content)
            current_section = line.strip('# ')
            current_content = []
        elif current_section:
            current_content.append(line)

    if current_section:
        sections[current_section] = '\n'.join(current_content)

    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("# Game Design Document Summary\n\n")
        f.write(f"Extracted: {datetime.now().strftime('%Y-%m-%d %H:%M')}\n\n")

        for section, body in sections.items():
            f.write(f"## {section}\n\n")
            # Take first 500 chars or full text
            if len(body) > 500:
                f.write(body[:500] + "...\n\n")
            else:
                f.write(body + "\n\n")

    print(f"✅ GDD summary saved to {output_file}")
    return True

def generate_architecture_diagram():
    """Generate text architecture diagram."""
    print("🏛️  Generating architecture diagram...")

    output_file = DOCS_DIR / "architecture.md"

    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("# Architecture Overview\n\n")
        f.write("## System Layers\n\n")

        f.write("```\n")
        f.write("┌─────────────────────────────────────────────┐\n")
        f.write("│              Game Layer (Bevy App)          │\n")
        f.write("├─────────────────────────────────────────────┤\n")
        f.write("│  UI Systems    │  Economy  │  AI Systems   │\n")
        f.write("│  Input Systems │  Combat   │  Animation     │\n")
        f.write("├─────────────────────────────────────────────┤\n")
        f.write("│         Shared Components (ECS)             │\n")
        f.write("│  - Hero, Enemy, Building, Merchant, etc.   │\n")
        f.write("├─────────────────────────────────────────────┤\n")
        f.write("│         Shared Events                       │\n")
        f.write("│  - BountyCompleted, HeroDeath, etc.        │\n")
        f.write("├─────────────────────────────────────────────┤\n")
        f.write("│         Resources (Global State)            │\n")
        f.write("│  - GameEconomy, GameTime, KingdomState     │\n")
        f.write("└─────────────────────────────────────────────┘\n")
        f.write("```\n\n")

        f.write("## Data Flow\n\n")
        f.write("1. **Player Input** → UI Events → System Queries\n")
        f.write("2. **Systems** read components, modify world, spawn events\n")
        f.write("3. **Events** trigger other systems, create effects\n")
        f.write("4. **Resources** provide global state access\n\n")

        f.write("## Core Systems\n\n")
        f.write("| System | File | Purpose |\n")
        f.write("|--------|------|---------|\n")
        f.write("| Hero AI | `hero.rs` | Hero decision-making |\n")
        f.write("| Enemy AI | `enemy.rs` | Enemy behavior |\n")
        f.write("| Combat | `combat.rs` | Attack/heal logic |\n")
        f.write("| Economy | `economy.rs` | Taxes, bounties |\n")
        f.write("| Spawning | `features.rs` | Dynamic entity creation |\n")
        f.write("| UI Update | `ui.rs` | HUD rendering |\n")

    print(f"✅ Architecture doc saved to {output_file}")
    return True

def run(args):
    """Run docs command."""
    actions = {
        'generate': [generate_component_docs, generate_api_docs, generate_gdd_summary, generate_architecture_diagram],
        'components': [generate_component_docs],
        'api': [generate_api_docs],
        'gdd': [generate_gdd_summary]
    }

    funcs = actions.get(args.action, [])
    if not funcs:
        print(f"Unknown action: {args.action}")
        return 1

    all_pass = True
    for func in funcs:
        try:
            if not func():
                all_pass = False
        except Exception as e:
            print(f"❌ Error in {func.__name__}: {e}")
            all_pass = False

    return 0 if all_pass else 1

if __name__ == "__main__":
    print("Docs module - use via 'python -m tools docs <action>'")
