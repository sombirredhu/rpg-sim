#!/usr/bin/env python3
"""
ECS (Entity Component System) analysis tools for Bevy.
Analyze components, systems, resources, and their relationships.
"""

import re
from pathlib import Path
from collections import defaultdict
import json

PROJECT_ROOT = Path(__file__).parent.parent.parent
SRC_DIR = PROJECT_ROOT / "src"

class ECSAnalyzer:
    """Analyze Bevy ECS structure from source code."""

    def __init__(self):
        self.components = {}          # name -> {file, fields, derives, etc.}
        self.systems = {}             # name -> {file, signature, etc.}
        self.resources = {}           # name -> {file, type, etc.}
        self.events = {}              # name -> {file, fields}
        self.system_deps = defaultdict(set)   # system -> components it uses
        self.component_users = defaultdict(set)  # component -> systems that use it

    def analyze_file(self, filepath):
        """Analyze a single Rust file for ECS elements."""
        try:
            content = filepath.read_text(encoding='utf-8')
        except Exception as e:
            print(f"⚠️  Could not read {filepath}: {e}")
            return

        filename = filepath.name

        # Find component definitions #[derive(Component)]
        component_pattern = r'#\[derive\([^)]*Component[^)]*\)\s*\]\s+(?:pub\s+)?struct\s+(\w+)'
        for match in re.finditer(component_pattern, content):
            name = match.group(1)
            derives = ""  # Not captured; could extract from full match if needed

            # Extract fields
            struct_start = match.end()
            brace_count = 0
            fields = []
            in_struct = False

            for i in range(struct_start, len(content)):
                if content[i] == '{':
                    in_struct = True
                    brace_count += 1
                elif content[i] == '}':
                    brace_count -= 1
                    if brace_count == 0 and in_struct:
                        break
                elif in_struct and brace_count == 1:
                    # This is a field line
                    line_start = content.rfind('\n', 0, i) + 1
                    line = content[line_start:i+1].strip()
                    if line and not line.startswith('//'):
                        fields.append(line)

            self.components[name] = {
                'file': filename,
                'fields': len(fields),
                'field_names': [f.split(':')[0].split()[0] for f in fields if ':' in f],
                'derives': derives
            }

        # Find system functions (fn with System trait or Query parameter)
        system_pattern = r'(?:pub\s+)?fn\s+(\w+)\s*\(([^)]*)\)'
        for match in re.finditer(system_pattern, content):
            name = match.group(1)
            params = match.group(2)

            # Check if it's a system (has Query, Commands, Res, etc.)
            if any(keyword in params for keyword in ['Query<', 'Commands', 'Res<', 'EventReader<', 'EventWriter<']):
                # Extract component types from Query
                queries = re.findall(r'Query<.*?(\w+)(?:<.*>)?>', params)

                self.systems[name] = {
                    'file': filename,
                    'params': params,
                    'queries': queries
                }

                # Track dependencies
                for comp in queries:
                    self.system_deps[name].add(comp)
                    self.component_users[comp].add(name)

        # Find resources #[derive(Resource)]
        resource_pattern = r'#\[derive\([^)]*Resource[^)]*\)\s*\]\s+(?:pub\s+)?(?:struct|enum)\s+(\w+)'
        for match in re.finditer(resource_pattern, content):
            name = match.group(1)
            self.resources[name] = {
                'file': filename,
                'derives': ""
            }

        # Find events
        event_pattern = r'#\[derive\([^)]*Event[^)]*\)\s*\]\s+pub\s+struct\s+(\w+)'
        for match in re.finditer(event_pattern, content):
            name = match.group(1)
            self.events[name] = {
                'file': filename,
                'derives': ""
            }

    def analyze_all(self):
        """Analyze all Rust source files."""
        print("🔍 Analyzing ECS structure...")

        for rust_file in SRC_DIR.glob("*.rs"):
            self.analyze_file(rust_file)

    def report_components(self):
        """Generate component report."""
        print("\n📦 COMPONENTS")
        print("-" * 80)

        by_file = defaultdict(list)
        for name, info in self.components.items():
            by_file[info['file']].append((name, info['fields']))

        for filename in sorted(by_file.keys()):
            print(f"\n{filename}:")
            for comp_name, field_count in sorted(by_file[filename], key=lambda x: x[1], reverse=True):
                field_info = f" ({field_count} fields)" if field_count > 0 else ""
                print(f"  ⊞ {comp_name}{field_info}")

        print(f"\nTotal: {len(self.components)} components")

    def report_systems(self):
        """Generate systems report."""
        print("\n⚙️  SYSTEMS")
        print("-" * 80)

        by_file = defaultdict(list)
        for name, info in self.systems.items():
            by_file[info['file']].append((name, info['queries']))

        for filename in sorted(by_file.keys()):
            print(f"\n{filename}:")
            for sys_name, queries in sorted(by_file[filename]):
                query_str = ', '.join(queries) if queries else "no queries"
                print(f"  ⚡ {sys_name:<30} uses: {query_str}")

        print(f"\nTotal: {len(self.systems)} systems")

    def report_resources(self):
        """Generate resources report."""
        print("\n💾 RESOURCES")
        print("-" * 80)

        by_file = defaultdict(list)
        for name, info in self.resources.items():
            by_file[info['file']].append(name)

        for filename in sorted(by_file.keys()):
            print(f"\n{filename}:")
            for res_name in sorted(by_file[filename]):
                print(f"  📁 {res_name}")

        print(f"\nTotal: {len(self.resources)} resources")

    def report_events(self):
        """Generate events report."""
        print("\n📡 EVENTS")
        print("-" * 80)

        by_file = defaultdict(list)
        for name, info in self.events.items():
            by_file[info['file']].append(name)

        for filename in sorted(by_file.keys()):
            print(f"\n{filename}:")
            for event_name in sorted(by_file[filename]):
                print(f"  🔔 {event_name}")

        print(f"\nTotal: {len(self.events)} events")

    def report_connections(self):
        """Show component-system connections."""
        print("\n🔗 COMPONENT USAGE")
        print("-" * 80)

        # Find most used components
        component_usage = [(c, len(users)) for c, users in self.component_users.items()]
        component_usage.sort(key=lambda x: x[1], reverse=True)

        print("\nTop 10 most-used components:")
        for comp, count in component_usage[:10]:
            print(f"  {comp:<30} used by {count:>2} system(s)")

        # Find orphaned components
        orphaned = [c for c in self.components if c not in self.component_users]
        if orphaned:
            print(f"\n⚠️  Orphaned components (no systems use them): {len(orphaned)}")
            for comp in sorted(orphaned)[:10]:
                print(f"  - {comp}")
            if len(orphaned) > 10:
                print(f"  ... and {len(orphaned) - 10} more")

        # Find systems with no component queries
        no_query = [s for s, deps in self.system_deps.items() if not deps]
        if no_query:
            print(f"\n⚠️  Systems with no component queries: {len(no_query)}")
            for sys in sorted(no_query):
                print(f"  - {sys}")

    def generate_full_report(self, output_path=None):
        """Generate complete ECS report."""
        self.analyze_all()

        report = {
            'summary': {
                'components': len(self.components),
                'systems': len(self.systems),
                'resources': len(self.resources),
                'events': len(self.events)
            },
            'components': self.components,
            'systems': self.systems,
            'resources': self.resources,
            'events': self.events,
            'component_usage': {c: list(users) for c, users in self.component_users.items()},
            'system_dependencies': dict(self.system_deps)
        }

        # Print to console
        self.report_components()
        self.report_systems()
        self.report_resources()
        self.report_events()
        self.report_connections()

        print("\n" + "="*80)
        print("SUMMARY")
        print("="*80)
        print(f"Components: {len(self.components)}")
        print(f"Systems: {len(self.systems)}")
        print(f"Resources: {len(self.resources)}")
        print(f"Events: {len(self.events)}")

        if output_path:
            output_path = Path(output_path)
            output_path.parent.mkdir(parents=True, exist_ok=True)
            with open(output_path, 'w') as f:
                json.dump(report, f, indent=2)
            print(f"\n📄 Full report saved to {output_path}")

        return report

def run(args):
    """Run ECS analysis."""
    analyzer = ECSAnalyzer()

    if args.report == 'components':
        analyzer.analyze_all()
        analyzer.report_components()
    elif args.report == 'systems':
        analyzer.analyze_all()
        analyzer.report_systems()
    elif args.report == 'resources':
        analyzer.analyze_all()
        analyzer.report_resources()
    elif args.report == 'report':
        analyzer.generate_full_report(args.output)

    return 0

if __name__ == "__main__":
    print("ECS module - use via 'python -m tools ecs <report>'")
