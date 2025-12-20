#!/usr/bin/env python3
"""
ANTLR Build Pipeline for Spark SQL Grammar

This script manages the complete build process:
1. Download grammar files from Apache Spark repository
2. Extract and analyze predicates (detects NEW predicates needing implementation)
3. Transform grammar for Rust target
4. Generate Rust code with ANTLR
5. Post-process to fix known issues

Usage:
    python build_antlr.py [command]

Commands:
    download    - Download latest grammar from Spark repo
    analyze     - Analyze predicates in grammar files (alerts on new ones)
    transform   - Transform grammar for Rust target
    generate    - Run ANTLR to generate Rust code
    postprocess - Apply post-processing fixes
    all         - Run complete pipeline (default)
    clean       - Remove generated files

Note: Predicate implementations are in src/antlr_predicates.rs (manually maintained).
"""

import json
import os
import re
import subprocess
import sys
import urllib.request
from pathlib import Path
from typing import Dict, List, Set, Tuple

# Configuration
SPARK_GRAMMAR_BASE_URL = "https://raw.githubusercontent.com/apache/spark/master/sql/api/src/main/antlr4/org/apache/spark/sql/catalyst/parser"
GRAMMAR_FILES = ["SqlBaseLexer.g4", "SqlBaseParser.g4"]
ANTLR_JAR = "antlr4-4.13.3-SNAPSHOT-complete.jar"  # The antlr4rust-specific JAR
ANTLR_JAR_URL = "https://github.com/antlr4rust/antlr4/releases/download/v0.5.0/antlr4-4.13.3-SNAPSHOT-complete.jar"

# Paths
SCRIPT_DIR = Path(__file__).parent
GRAMMAR_SOURCE_DIR = SCRIPT_DIR.parent.parent / "grammar"
GRAMMAR_TRANSFORMED_DIR = SCRIPT_DIR / "grammar"
GENERATED_DIR = SCRIPT_DIR / "src" / "generated"
PREDICATES_FILE = SCRIPT_DIR / "KNOWN_PREDICATES.json"


def download_grammar():
    """Download latest grammar files from Apache Spark repository."""
    print("=" * 60)
    print("Downloading grammar files from Apache Spark...")
    print("=" * 60)
    
    GRAMMAR_SOURCE_DIR.mkdir(parents=True, exist_ok=True)
    
    for filename in GRAMMAR_FILES:
        url = f"{SPARK_GRAMMAR_BASE_URL}/{filename}"
        output_path = GRAMMAR_SOURCE_DIR / filename
        
        print(f"  Downloading {filename}...")
        try:
            urllib.request.urlretrieve(url, output_path)
            print(f"    -> Saved to {output_path}")
        except Exception as e:
            print(f"    ERROR: Failed to download {filename}: {e}")
            return False
    
    # Update version file
    import datetime
    version_file = GRAMMAR_SOURCE_DIR / "VERSION"
    version_file.write_text(f"spark-master-{datetime.datetime.now().strftime('%Y%m%d')}\n")
    print(f"  Updated VERSION file")
    
    # Download ANTLR JAR if not present
    jar_path = SCRIPT_DIR / ANTLR_JAR
    if not jar_path.exists():
        print(f"\n  Downloading {ANTLR_JAR}...")
        try:
            urllib.request.urlretrieve(ANTLR_JAR_URL, jar_path)
            print(f"    -> Saved to {jar_path}")
        except Exception as e:
            print(f"    ERROR: Failed to download {ANTLR_JAR}: {e}")
            print(f"    Manual download: {ANTLR_JAR_URL}")
            return False
    else:
        print(f"\n  {ANTLR_JAR} already present")
    
    return True


def extract_predicates_from_grammar(content: str) -> Dict[str, List[str]]:
    """Extract all predicate patterns from grammar content."""
    predicates = {
        "method_predicates": [],      # {methodName()}?
        "negated_predicates": [],     # {!methodName()}?
        "variable_predicates": [],    # {variableName}?
        "actions": [],                # {methodName();}
        "special_patterns": []        # Other patterns like tags.push()
    }
    
    # Method predicates: {methodName()}? or {methodName(args)}?
    for match in re.finditer(r'\{(\w+)\(([^)]*)\)\}\?', content):
        predicates["method_predicates"].append(match.group(1))
    
    # Negated predicates: {!methodName()}?
    for match in re.finditer(r'\{!(\w+)\(([^)]*)\)\}\?', content):
        predicates["negated_predicates"].append(match.group(1))
    
    # Variable predicates: {variableName}? (not followed by parentheses)
    for match in re.finditer(r'\{!?([a-zA-Z][a-zA-Z_0-9]*)\}\?', content):
        if match.group(1) not in ['true', 'false']:
            predicates["variable_predicates"].append(match.group(1))
    
    # Actions: {methodName();}
    for match in re.finditer(r'\{(\w+)\(\);?\}(?!\?)', content):
        predicates["actions"].append(match.group(1))
    
    # Special patterns
    if 'tags.push' in content:
        predicates["special_patterns"].append("tags.push(getText())")
    if 'tags.pop' in content:
        predicates["special_patterns"].append("tags.pop()")
    if 'tags.peek' in content:
        predicates["special_patterns"].append("getText().equals(tags.peek())")
    
    # Deduplicate while preserving order
    for key in predicates:
        predicates[key] = list(dict.fromkeys(predicates[key]))
    
    return predicates


def analyze_predicates():
    """Analyze predicates in grammar files and compare with known predicates."""
    print("=" * 60)
    print("Analyzing predicates in grammar files...")
    print("=" * 60)
    
    # Load known predicates
    known = {}
    if PREDICATES_FILE.exists():
        with open(PREDICATES_FILE) as f:
            known = json.load(f)
    
    # Build set of all known predicate names (snake_case)
    all_known = set()
    # Map of original patterns to their snake_case names
    special_pattern_map = {}
    
    for section in ['lexer_predicates', 'parser_predicates']:
        if section in known:
            for subsection in ['methods', 'actions', 'config_flags']:
                if subsection in known[section]:
                    for key, info in known[section][subsection].items():
                        all_known.add(key)
                        # Also track original patterns for special cases
                        if 'original' in info:
                            special_pattern_map[info['original']] = key
    
    found_predicates = {"lexer": {}, "parser": {}}
    new_predicates = []
    
    for filename in GRAMMAR_FILES:
        source_path = GRAMMAR_SOURCE_DIR / filename
        if not source_path.exists():
            print(f"  WARNING: {source_path} not found. Run 'download' first.")
            continue
        
        content = source_path.read_text(encoding='utf-8')
        predicates = extract_predicates_from_grammar(content)
        
        grammar_type = "lexer" if "Lexer" in filename else "parser"
        found_predicates[grammar_type] = predicates
        
        print(f"\n  {filename}:")
        for category, items in predicates.items():
            if items:
                print(f"    {category}: {items}")
                
                # Check for new predicates
                for item in items:
                    snake_case = to_snake_case(item)
                    # Check if known by snake_case name, original name, or special pattern
                    is_known = (
                        snake_case in all_known or 
                        item in all_known or
                        item in special_pattern_map
                    )
                    if not is_known:
                        new_predicates.append((filename, category, item))
    
    if new_predicates:
        print("\n" + "!" * 60)
        print("NEW PREDICATES DETECTED - Update KNOWN_PREDICATES.json!")
        print("!" * 60)
        for filename, category, predicate in new_predicates:
            print(f"  [{filename}] {category}: {predicate}")
        return False
    else:
        print("\n  ✓ All predicates are known")
    
    return True


def to_snake_case(name: str) -> str:
    """Convert camelCase or PascalCase to snake_case."""
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


def remove_block(content: str, block_name: str) -> str:
    """Remove @header or @members blocks with their braced content."""
    pattern = rf'@{block_name}\s*\{{'
    
    result = []
    i = 0
    while i < len(content):
        match = re.match(pattern, content[i:])
        if match:
            brace_count = 1
            j = i + match.end()
            while j < len(content) and brace_count > 0:
                if content[j] == '{':
                    brace_count += 1
                elif content[j] == '}':
                    brace_count -= 1
                j += 1
            while j < len(content) and content[j] in ' \t\n\r':
                j += 1
            i = j
        else:
            result.append(content[i])
            i += 1
    
    return ''.join(result)


def transform_predicates(content: str) -> str:
    """Transform Java predicate/action syntax to Rust."""
    
    # Special case: dollar-quoted string tag handling
    content = re.sub(
        r'\{tags\.push\(getText\(\)\);?\}',
        '{recog.push_dollar_tag();}',
        content
    )
    content = re.sub(
        r'\{getText\(\)\.equals\(tags\.peek\(\)\)\}\?',
        '{recog.matches_dollar_tag()}?',
        content
    )
    content = re.sub(
        r'\{tags\.pop\(\);?\}',
        '{recog.pop_dollar_tag();}',
        content
    )
    
    # Transform method calls in predicates: {methodName()}? or {!methodName()}?
    content = re.sub(
        r'\{(!?)(\w+)\(([^)]*)\)\}(\?)?',
        lambda m: '{' + m.group(1) + 'recog.' + to_snake_case(m.group(2)) + '(' + m.group(3) + ')' + '}' + (m.group(4) or ''),
        content
    )
    
    # Transform method calls in actions: {methodName();}
    content = re.sub(
        r'\{(\w+)\(\);?\}',
        lambda m: '{recog.' + to_snake_case(m.group(1)) + '();}',
        content
    )
    
    # Transform variable access in predicates: {variableName}? or {!variableName}?
    content = re.sub(
        r'\{(!?)([a-zA-Z][a-zA-Z_0-9]*)\}(\?)',
        lambda m: '{' + m.group(1) + 'recog.' + to_snake_case(m.group(2)) + '}' + m.group(3),
        content
    )
    
    return content


def transform_grammar():
    """Transform grammar files for Rust ANTLR target."""
    print("=" * 60)
    print("Transforming grammar for Rust target...")
    print("=" * 60)
    
    GRAMMAR_TRANSFORMED_DIR.mkdir(parents=True, exist_ok=True)
    
    for filename in GRAMMAR_FILES:
        source_path = GRAMMAR_SOURCE_DIR / filename
        output_path = GRAMMAR_TRANSFORMED_DIR / filename
        
        if not source_path.exists():
            print(f"  ERROR: {source_path} not found")
            return False
        
        content = source_path.read_text(encoding='utf-8')
        
        # Remove Java-specific blocks
        content = remove_block(content, 'header')
        content = remove_block(content, 'members')
        
        # Transform predicate/action syntax for Rust
        content = transform_predicates(content)
        
        output_path.write_text(content, encoding='utf-8')
        print(f"  Transformed: {filename}")
    
    return True


def generate_antlr():
    """Run ANTLR to generate Rust lexer/parser."""
    print("=" * 60)
    print("Generating Rust code with ANTLR...")
    print("=" * 60)
    
    GENERATED_DIR.mkdir(parents=True, exist_ok=True)
    
    jar_path = SCRIPT_DIR / ANTLR_JAR
    if not jar_path.exists():
        print(f"  ERROR: {ANTLR_JAR} not found. Download from:")
        print(f"    https://github.com/AntlrRustRuntime/antlr4rust/releases")
        return False
    
    # Generate from both grammar files
    for filename in GRAMMAR_FILES:
        grammar_path = GRAMMAR_TRANSFORMED_DIR / filename
        if not grammar_path.exists():
            print(f"  ERROR: {grammar_path} not found. Run 'transform' first.")
            return False
    
    # Run ANTLR
    cmd = [
        "java", "-jar", str(jar_path),
        "-Dlanguage=Rust",
        "-o", str(GENERATED_DIR),
        str(GRAMMAR_TRANSFORMED_DIR / "SqlBaseLexer.g4"),
        str(GRAMMAR_TRANSFORMED_DIR / "SqlBaseParser.g4"),
    ]
    
    print(f"  Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    if result.returncode != 0:
        print(f"  ANTLR STDERR:\n{result.stderr}")
        print(f"  ERROR: ANTLR generation failed")
        return False
    
    if result.stderr:
        print(f"  ANTLR warnings:\n{result.stderr}")
    
    print(f"  Generated files in {GENERATED_DIR}")
    return True


def postprocess_generated():
    """Apply post-processing fixes to generated code."""
    print("=" * 60)
    print("Post-processing generated code...")
    print("=" * 60)
    
    fixes_applied = 0
    
    # Parser config predicates that are accessed as fields in the grammar
    # but we implement them as methods (via extension traits)
    PARSER_CONFIG_PREDICATES = [
        'legacy_setops_precedence_enabled',
        'sql_standard_keyword_behavior',
        'double_quoted_identifiers',
        'parameter_substitution_enabled',
        'legacy_exponent_literal_as_decimal_enabled',
        'legacy_identifier_clause_only',
    ]
    
    for rs_file in GENERATED_DIR.glob("*.rs"):
        content = rs_file.read_text(encoding='utf-8')
        original = content
        
        # Fix 1: SqlBaseParserParserContext -> SqlBaseParserContext
        # This is a known bug in antlr4rust generator
        content = content.replace("SqlBaseParserParserContext", "SqlBaseParserContext")
        
        # Fix 2a: Duplicate 'base' field in struct declarations
        # Pattern: struct has base:SomeContextExt<'input>, then pub base: Option<...>
        # We need to rename the second one to parent_ctx
        content = re.sub(
            r"(base:\w+ContextExt<'input>,\s*\n\s*)pub base:",
            r"\1pub parent_ctx:",
            content
        )
        
        # Fix 2b: Duplicate 'base' in struct initialization (copy_from calls)
        # Pattern: base:None, fieldName:..., \n base: ctx.borrow().clone()
        # The first `base:None` should become `parent_ctx:None`
        content = re.sub(
            r"base:None,(\s*\w+:.*?,\s*\n\s*)base:",
            r"parent_ctx:None,\1base:",
            content
        )
        
        # Fix 2c: Incorrect field assignment in recursion context
        # Pattern: ctx.base = Some(_prevctx.clone()) should be ctx.parent_ctx = ...
        # This happens in left-recursive rule handling
        content = re.sub(
            r"ctx\.base = Some\(_prevctx\.clone\(\)\)",
            r"ctx.parent_ctx = Some(_prevctx.clone())",
            content
        )
        
        # Fix 3: Add predicate trait imports to lexer
        # The extension trait must be in scope for methods to resolve
        if rs_file.name == "sqlbaselexer.rs":
            if "use crate::antlr4rust_workarounds::LexerPredicates;" not in content:
                # Add import after the existing use statements
                content = re.sub(
                    r"(use antlr4rust::[^\n]+;\n)",
                    r"\1use crate::antlr4rust_workarounds::LexerPredicates;\n",
                    content,
                    count=1  # Only add once
                )
        
        # Fix 4: Add predicate trait imports to parser + convert field access to method calls
        if rs_file.name == "sqlbaseparser.rs":
            if "use crate::antlr4rust_workarounds::ParserPredicates;" not in content:
                content = re.sub(
                    r"(use antlr4rust::[^\n]+;\n)",
                    r"\1use crate::antlr4rust_workarounds::ParserPredicates;\n",
                    content,
                    count=1
                )
            
            # Fix 5: Convert parser config field accesses to method calls
            # The grammar has predicates like {legacy_setops_precedence_enabled}?
            # which generates field access, but we implement them as methods
            for pred in PARSER_CONFIG_PREDICATES:
                # Match patterns like `recog.pred` or `!recog.pred` and add ()
                # Be careful not to match method calls that already have ()
                content = re.sub(
                    rf'(recog\.{pred})([^(\w])',
                    rf'\1()\2',
                    content
                )
        
        if content != original:
            rs_file.write_text(content, encoding='utf-8')
            print(f"  Fixed: {rs_file.name}")
            fixes_applied += 1
    
    # Create or update mod.rs
    mod_rs = GENERATED_DIR / "mod.rs"
    modules = []
    for rs_file in GENERATED_DIR.glob("*.rs"):
        if rs_file.name != "mod.rs":
            mod_name = rs_file.stem
            modules.append(f"pub mod {mod_name};")
    
    mod_content = "//! ANTLR4 Generated Spark SQL Parser\n"
    mod_content += "//!\n"
    mod_content += "//! This module contains the generated lexer, parser, and listener code.\n"
    mod_content += "//! DO NOT EDIT - Generated by build_antlr.py\n\n"
    mod_content += "\n".join(sorted(modules)) + "\n"
    
    if not mod_rs.exists() or mod_rs.read_text() != mod_content:
        mod_rs.write_text(mod_content, encoding='utf-8')
        print(f"  Updated: mod.rs")
        fixes_applied += 1
    
    print(f"  Applied {fixes_applied} fixes")
    return True


def clean():
    """Remove generated files."""
    print("=" * 60)
    print("Cleaning generated files...")
    print("=" * 60)
    
    import shutil
    
    if GENERATED_DIR.exists():
        shutil.rmtree(GENERATED_DIR)
        print(f"  Removed: {GENERATED_DIR}")
    
    if GRAMMAR_TRANSFORMED_DIR.exists():
        shutil.rmtree(GRAMMAR_TRANSFORMED_DIR)
        print(f"  Removed: {GRAMMAR_TRANSFORMED_DIR}")
    
    jar_path = SCRIPT_DIR / ANTLR_JAR
    if jar_path.exists():
        jar_path.unlink()
        print(f"  Removed: {jar_path}")
    
    return True


def run_all():
    """Run complete build pipeline."""
    steps = [
        ("Download grammar", download_grammar),
        ("Analyze predicates", analyze_predicates),
        ("Transform grammar", transform_grammar),
        ("Generate ANTLR", generate_antlr),
        ("Post-process", postprocess_generated),
    ]
    
    for name, func in steps:
        if not func():
            print(f"\n❌ Pipeline failed at: {name}")
            return False
        print()
    
    print("=" * 60)
    print("✓ Build pipeline completed successfully!")
    print("=" * 60)
    return True


def main():
    commands = {
        "download": download_grammar,
        "analyze": analyze_predicates,
        "transform": transform_grammar,
        "generate": generate_antlr,
        "postprocess": postprocess_generated,
        "all": run_all,
        "clean": clean,
    }
    
    if len(sys.argv) < 2:
        command = "all"
    else:
        command = sys.argv[1]
    
    if command in ["-h", "--help", "help"]:
        print(__doc__)
        return 0
    
    if command not in commands:
        print(f"Unknown command: {command}")
        print(f"Available commands: {', '.join(commands.keys())}")
        return 1
    
    success = commands[command]()
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())
