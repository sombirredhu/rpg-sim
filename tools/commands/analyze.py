#!/usr/bin/env python3
"""
Code analysis tools for the Bevy project.
Analyzes code structure, complexity, dependencies, and architecture.
"""

import re
from pathlib import Path
from collections import defaultdict, Counter
import json

PROJECT_ROOT = Path(__file__).parent.parent.parent
SRC_DIR = PROJECT_ROOT / "src"

class CodeAnalyzer:
    """Analyze Rust source code structure and quality."""

    def __init__(self):
        self.files = {}
        self.stats = defaultdict(dict)
        self.dependencies = defaultdict(set)

    def load_files(self):
        """Load all Rust source files."""
        for rust_file in SRC_DIR.glob("*.rs"):
            try:
                content = rust_file.read_text(encoding='utf-8')
                self.files[rust_file.name] = content
            except Exception as e:
                print(f"⚠️  Could not read {rust_file}: {e}")

    def analyze_structure(self):
        """Analyze file structure and organization."""
        print("📊 Analyzing code structure...")

        for filename, content in self.files.items():
            lines = content.split('\n')

            # Count various elements
            self.stats[filename]['lines'] = len(lines)
            self.stats[filename]['comments'] = sum(1 for l in lines if l.strip().startswith('//'))
            self.stats[filename]['blank'] = sum(1 for l in lines if not l.strip())

            # Count imports
            imports = re.findall(r'use\s+([^;]+);', content)
            self.stats[filename]['imports'] = len(imports)
            self.stats[filename]['import_list'] = imports

            # Count functions and structs
            self.stats[filename]['functions'] = len(re.findall(r'fn\s+\w+', content))
            self.stats[filename]['structs'] = len(re.findall(r'(?:pub\s+)?struct\s+\w+', content))
            self.stats[filename]['impls'] = len(re.findall(r'impl\s+\w+', content))

            # Extract module definition
            if 'mod ' in content:
                self.stats[filename]['is_module'] = True

    def analyze_complexity(self):
        """Check complexity indicators."""
        print("⚖️  Analyzing complexity...")

        for filename, content in self.files.items():
            # Nesting depth
            max_indent = 0
            for line in content.split('\n'):
                if line.strip() and not line.strip().startswith('//'):
                    indent = len(line) - len(line.lstrip())
                    max_indent = max(max_indent, indent)

            self.stats[filename]['max_indent'] = max_indent // 4  # Convert to 4-space blocks

            # Large match/if blocks
            match_blocks = re.findall(r'match\s+\w+.*?\{', content, re.DOTALL)
            if match_blocks:
                self.stats[filename]['large_matches'] = sum(
                    1 for m in match_blocks if m.count('\n') > 20
                )

    def analyze_dependencies(self):
        """Build dependency graph between modules."""
        print("🔗 Analyzing dependencies...")

        for filename, content in self.files.items():
            imports = re.findall(r'use\s+([^;]+);', content)
            for imp in imports:
                # Extract module path
                parts = imp.strip().split('::')
                if parts[0] == 'super' or parts[0] == 'crate':
                    continue
                if len(parts) >= 1:
                    self.dependencies[filename].add(parts[0])

    def generate_report(self, output_path=None):
        """Generate analysis report."""
        report = {
            'summary': {
                'total_files': len(self.files),
                'total_lines': sum(s['lines'] for s in self.stats.values()),
                'total_functions': sum(s.get('functions', 0) for s in self.stats.values()),
                'total_structs': sum(s.get('structs', 0) for s in self.stats.values()),
            },
            'files': self.stats,
            'dependencies': dict(self.dependencies),
            'warnings': []
        }

        # Check for warnings
        for filename, stats in self.stats.items():
            if stats.get('max_indent', 0) > 4:
                report['warnings'].append(
                    f"{filename}: Deep nesting (max {stats['max_indent']} levels)"
                )
            if stats.get('lines', 0) > 800:
                report['warnings'].append(
                    f"{filename}: Large file ({stats['lines']} lines)"
                )
            if stats.get('imports', 0) > 30:
                report['warnings'].append(
                    f"{filename}: Many imports ({stats['imports']})"
                )

        if output_path:
            output_path = Path(output_path)
            output_path.parent.mkdir(parents=True, exist_ok=True)
            with open(output_path, 'w') as f:
                json.dump(report, f, indent=2)
            print(f"📄 Report saved to {output_path}")
        else:
            print("\n" + "="*60)
            print("CODE ANALYSIS REPORT")
            print("="*60)
            print(f"📦 Total files: {report['summary']['total_files']}")
            print(f"📝 Total lines: {report['summary']['total_lines']}")
            print(f"🔧 Functions: {report['summary']['total_functions']}")
            print(f"📋 Structs: {report['summary']['total_structs']}")
            print("\n📊 File breakdown:")
            for filename, stats in sorted(self.stats.items(), key=lambda x: x[1]['lines'], reverse=True):
                print(f"  {filename:25} {stats['lines']:5} lines | "
                      f"{stats.get('functions', 0):3} fn | "
                      f"{stats.get('structs', 0):3} struct | "
                      f"{stats.get('impls', 0):3} impl")

            if report['warnings']:
                print("\n⚠️  Warnings:")
                for w in report['warnings']:
                    print(f"  - {w}")

        return report

def run(args):
    """Run analysis based on arguments."""
    analyzer = CodeAnalyzer()
    analyzer.load_files()

    if args.type == 'all':
        analyzer.analyze_structure()
        analyzer.analyze_complexity()
        analyzer.analyze_dependencies()
        report = analyzer.generate_report(args.output)
        return 0
    elif args.type == 'structure':
        analyzer.analyze_structure()
        report = analyzer.generate_report(args.output)
        return 0
    elif args.type == 'complexity':
        analyzer.load_files()
        analyzer.analyze_complexity()
        report = analyzer.generate_report(args.output)
        return 0
    elif args.type == 'deps':
        analyzer.load_files()
        analyzer.analyze_dependencies()
        report = analyzer.generate_report(args.output)
        return 0

    return 0

if __name__ == "__main__":
    print("Analysis module - use via 'python -m tools analyze <type>'")
