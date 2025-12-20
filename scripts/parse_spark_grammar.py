#!/usr/bin/env python3
"""
Parse Apache Spark's ANTLR grammar files and generate comprehensive test SQL.

This script:
1. Downloads/reads SqlBaseLexer.g4 and SqlBaseParser.g4
2. Extracts keywords, operators, and grammar patterns
3. Generates test SQL files covering all constructs
4. Compares against current implementation to find gaps
"""

import re
import json
import urllib.request
from pathlib import Path
from typing import Dict, List, Set, Tuple

# URLs for Spark grammar files
LEXER_URL = "https://raw.githubusercontent.com/apache/spark/master/sql/api/src/main/antlr4/org/apache/spark/sql/catalyst/parser/SqlBaseLexer.g4"
PARSER_URL = "https://raw.githubusercontent.com/apache/spark/master/sql/api/src/main/antlr4/org/apache/spark/sql/catalyst/parser/SqlBaseParser.g4"

# Output directories
SCRIPT_DIR = Path(__file__).parent
PROJECT_ROOT = SCRIPT_DIR.parent
TEST_DATA_DIR = PROJECT_ROOT / "test_data" / "grammar_coverage"
OUTPUT_JSON = PROJECT_ROOT / "grammar_analysis.json"


def download_grammar(url: str) -> str:
    """Download grammar file from GitHub."""
    print(f"Downloading {url}...")
    with urllib.request.urlopen(url) as response:
        return response.read().decode('utf-8')


def extract_keywords_from_lexer(lexer_content: str) -> List[str]:
    """
    Extract keywords from SqlBaseLexer.g4.
    Keywords are between //--SPARK-KEYWORD-LIST-START and //--SPARK-KEYWORD-LIST-END
    """
    keywords = []
    
    # Find the keyword section
    start_marker = "//--SPARK-KEYWORD-LIST-START"
    end_marker = "//--SPARK-KEYWORD-LIST-END"
    
    start_idx = lexer_content.find(start_marker)
    end_idx = lexer_content.find(end_marker)
    
    if start_idx == -1 or end_idx == -1:
        print("Warning: Could not find keyword markers in lexer")
        # Fallback: extract all token definitions that look like keywords
        keyword_pattern = re.compile(r"^([A-Z_]+)\s*:\s*'[A-Za-z_]+'", re.MULTILINE)
        for match in keyword_pattern.finditer(lexer_content):
            keywords.append(match.group(1))
        return keywords
    
    keyword_section = lexer_content[start_idx:end_idx]
    
    # Extract keyword definitions: KEYWORD: 'keyword';
    keyword_pattern = re.compile(r"^([A-Z_][A-Z_0-9]*)\s*:", re.MULTILINE)
    for match in keyword_pattern.finditer(keyword_section):
        kw = match.group(1)
        # Skip internal tokens
        if not kw.startswith('_'):
            keywords.append(kw)
    
    return sorted(set(keywords))


def extract_operators_from_lexer(lexer_content: str) -> Dict[str, str]:
    """Extract operator definitions from lexer."""
    operators = {}
    
    # Common operator patterns
    op_patterns = [
        (r"EQ\s*:\s*'='", "EQ", "="),
        (r"NSEQ\s*:\s*'<=>'", "NSEQ", "<=>"),
        (r"NEQ\s*:\s*'<>'", "NEQ", "<>"),
        (r"NEQJ\s*:\s*'!='", "NEQJ", "!="),
        (r"LTE\s*:\s*'<='", "LTE", "<="),
        (r"LT\s*:\s*'<'", "LT", "<"),
        (r"GTE\s*:\s*'>='", "GTE", ">="),
        (r"GT\s*:\s*'>'", "GT", ">"),
        (r"SHIFT_LEFT\s*:\s*'<<'", "SHIFT_LEFT", "<<"),
        (r"SHIFT_RIGHT_UNSIGNED\s*:\s*'>>>'", "SHIFT_RIGHT_UNSIGNED", ">>>"),
        (r"SHIFT_RIGHT\s*:\s*'>>'", "SHIFT_RIGHT", ">>"),
        (r"PLUS\s*:\s*'\+'", "PLUS", "+"),
        (r"MINUS\s*:\s*'-'", "MINUS", "-"),
        (r"ASTERISK\s*:\s*'\*'", "ASTERISK", "*"),
        (r"SLASH\s*:\s*'/'", "SLASH", "/"),
        (r"PERCENT\s*:\s*'%'", "PERCENT", "%"),
        (r"TILDE\s*:\s*'~'", "TILDE", "~"),
        (r"AMPERSAND\s*:\s*'&'", "AMPERSAND", "&"),
        (r"CONCAT_PIPE\s*:\s*'\|\|'", "CONCAT_PIPE", "||"),
        (r"OPERATOR_PIPE\s*:\s*'\|>'", "OPERATOR_PIPE", "|>"),
        (r"HAT\s*:\s*'\^'", "HAT", "^"),
        (r"DOUBLE_COLON\s*:\s*'::'", "DOUBLE_COLON", "::"),
        (r"COLON\s*:\s*':'", "COLON", ":"),
        (r"ARROW\s*:\s*'->'", "ARROW", "->"),
        (r"FAT_ARROW\s*:\s*'=>'", "FAT_ARROW", "=>"),
    ]
    
    for pattern, name, symbol in op_patterns:
        if re.search(pattern, lexer_content):
            operators[name] = symbol
    
    return operators


def extract_literal_patterns(lexer_content: str) -> Dict[str, str]:
    """Extract number literal patterns."""
    literals = {
        "INTEGER_VALUE": "123",
        "DECIMAL_VALUE": "123.456",
        "EXPONENT_VALUE": "1.5e10",
        "BIGINT_LITERAL": "100L",
        "SMALLINT_LITERAL": "50S",
        "TINYINT_LITERAL": "10Y",
        "FLOAT_LITERAL": "3.14F",
        "DOUBLE_LITERAL": "2.718D",
        "BIGDECIMAL_LITERAL": "99.99BD",
        "BINARY_HEX": "X'1F2A'",
    }
    return literals


def read_current_keywords(project_root: Path) -> Set[str]:
    """Read keywords from current keywords.rs."""
    keywords_file = project_root / "crates" / "sparkfmt-core" / "src" / "keywords.rs"
    
    if not keywords_file.exists():
        return set()
    
    content = keywords_file.read_text()
    
    # Extract keywords from set.insert("KEYWORD");
    pattern = re.compile(r'set\.insert\("([A-Z_]+)"\)')
    return set(pattern.findall(content))


def generate_keyword_tests(keywords: List[str]) -> str:
    """Generate test SQL for keyword handling."""
    tests = []
    tests.append("-- Grammar-generated keyword tests")
    tests.append("-- Each keyword tested as: keyword in context, keyword as quoted identifier")
    tests.append("")
    
    # Group tests to keep file manageable
    for kw in keywords[:50]:  # First 50 keywords for basic test
        tests.append(f"-- Test keyword: {kw}")
        tests.append(f"SELECT `{kw.lower()}` FROM t")
    
    return "\n".join(tests)


def generate_operator_tests(operators: Dict[str, str]) -> str:
    """Generate test SQL for operators."""
    tests = []
    tests.append("-- Grammar-generated operator tests")
    tests.append("")
    
    for name, symbol in operators.items():
        tests.append(f"-- Operator: {name} ({symbol})")
        
        if name in ["PLUS", "MINUS", "ASTERISK", "SLASH", "PERCENT", "DIV"]:
            tests.append(f"SELECT a {symbol} b FROM t")
        elif name in ["EQ", "NEQ", "NEQJ", "LT", "LTE", "GT", "GTE", "NSEQ"]:
            tests.append(f"SELECT * FROM t WHERE a {symbol} b")
        elif name in ["AND", "OR"]:
            tests.append(f"SELECT * FROM t WHERE a = 1 {symbol} b = 2")
        elif name == "CONCAT_PIPE":
            tests.append(f"SELECT a {symbol} b FROM t")
        elif name == "DOUBLE_COLON":
            tests.append(f"SELECT x{symbol}INT FROM t")
        elif name == "ARROW":
            tests.append(f"SELECT transform(arr, x {symbol} x + 1) FROM t")
        elif name == "FAT_ARROW":
            tests.append(f"SELECT func(param {symbol} value) FROM t")
        else:
            tests.append(f"SELECT a {symbol} b FROM t")
        tests.append("")
    
    return "\n".join(tests)


def generate_literal_tests(literals: Dict[str, str]) -> str:
    """Generate test SQL for literal formats."""
    tests = []
    tests.append("-- Grammar-generated literal format tests")
    tests.append("")
    
    # Scientific notation variations
    tests.append("-- Scientific notation")
    tests.append("SELECT 1e10 FROM t")
    tests.append("SELECT 1.5e10 FROM t")
    tests.append("SELECT 1E-5 FROM t")
    tests.append("SELECT 1.5E+10 FROM t")
    tests.append("SELECT -1e10 FROM t")
    tests.append("")
    
    # Suffixed literals
    tests.append("-- Suffixed numeric literals")
    tests.append("SELECT 100L FROM t")
    tests.append("SELECT 50S FROM t")
    tests.append("SELECT 10Y FROM t")
    tests.append("SELECT 3.14F FROM t")
    tests.append("SELECT 2.718D FROM t")
    tests.append("SELECT 99.99BD FROM t")
    tests.append("")
    
    # Hex literals
    tests.append("-- Hex literals")
    tests.append("SELECT X'1F2A' FROM t")
    tests.append("SELECT X'' FROM t")
    tests.append("SELECT X'DEADBEEF' FROM t")
    tests.append("")
    
    return "\n".join(tests)


def generate_clause_tests() -> str:
    """Generate test SQL for clause combinations."""
    tests = []
    tests.append("-- Grammar-generated clause combination tests")
    tests.append("")
    
    # Query organization clauses
    tests.append("-- ORDER BY variations")
    tests.append("SELECT * FROM t ORDER BY x")
    tests.append("SELECT * FROM t ORDER BY x ASC")
    tests.append("SELECT * FROM t ORDER BY x DESC")
    tests.append("SELECT * FROM t ORDER BY x NULLS FIRST")
    tests.append("SELECT * FROM t ORDER BY x NULLS LAST")
    tests.append("SELECT * FROM t ORDER BY x ASC NULLS FIRST")
    tests.append("")
    
    tests.append("-- CLUSTER BY (Spark-specific)")
    tests.append("SELECT * FROM t CLUSTER BY x")
    tests.append("SELECT * FROM t CLUSTER BY x, y")
    tests.append("")
    
    tests.append("-- DISTRIBUTE BY (Spark-specific)")
    tests.append("SELECT * FROM t DISTRIBUTE BY x")
    tests.append("SELECT * FROM t DISTRIBUTE BY x, y")
    tests.append("")
    
    tests.append("-- SORT BY (Spark-specific)")
    tests.append("SELECT * FROM t SORT BY x")
    tests.append("SELECT * FROM t SORT BY x ASC")
    tests.append("")
    
    tests.append("-- Combined DISTRIBUTE BY and SORT BY")
    tests.append("SELECT * FROM t DISTRIBUTE BY x SORT BY y")
    tests.append("")
    
    tests.append("-- LIMIT and OFFSET")
    tests.append("SELECT * FROM t LIMIT 10")
    tests.append("SELECT * FROM t LIMIT 10 OFFSET 5")
    tests.append("SELECT * FROM t LIMIT ALL")
    tests.append("")
    
    # Set operations
    tests.append("-- Set operations")
    tests.append("SELECT * FROM a UNION SELECT * FROM b")
    tests.append("SELECT * FROM a UNION ALL SELECT * FROM b")
    tests.append("SELECT * FROM a UNION DISTINCT SELECT * FROM b")
    tests.append("SELECT * FROM a EXCEPT SELECT * FROM b")
    tests.append("SELECT * FROM a EXCEPT ALL SELECT * FROM b")
    tests.append("SELECT * FROM a INTERSECT SELECT * FROM b")
    tests.append("SELECT * FROM a INTERSECT ALL SELECT * FROM b")
    tests.append("SELECT * FROM a MINUS SELECT * FROM b")
    tests.append("")
    
    # QUALIFY clause
    tests.append("-- QUALIFY clause (window function filter)")
    # Note: QUALIFY is not yet in grammar queryOrganization, but is a Spark feature
    
    return "\n".join(tests)


def generate_join_tests() -> str:
    """Generate test SQL for all join types."""
    tests = []
    tests.append("-- Grammar-generated join type tests")
    tests.append("")
    
    join_types = [
        ("JOIN", "INNER JOIN (implicit)"),
        ("INNER JOIN", "INNER JOIN (explicit)"),
        ("LEFT JOIN", "LEFT JOIN"),
        ("LEFT OUTER JOIN", "LEFT OUTER JOIN"),
        ("RIGHT JOIN", "RIGHT JOIN"),
        ("RIGHT OUTER JOIN", "RIGHT OUTER JOIN"),
        ("FULL JOIN", "FULL JOIN"),
        ("FULL OUTER JOIN", "FULL OUTER JOIN"),
        ("CROSS JOIN", "CROSS JOIN (no ON clause)"),
        ("LEFT SEMI JOIN", "LEFT SEMI JOIN"),
        ("LEFT ANTI JOIN", "LEFT ANTI JOIN"),
        ("NATURAL JOIN", "NATURAL JOIN"),
        ("NATURAL LEFT JOIN", "NATURAL LEFT JOIN"),
        ("NATURAL RIGHT JOIN", "NATURAL RIGHT JOIN"),
        ("NATURAL FULL JOIN", "NATURAL FULL JOIN"),
    ]
    
    for join_syntax, description in join_types:
        tests.append(f"-- {description}")
        if "CROSS" in join_syntax or "NATURAL" in join_syntax:
            tests.append(f"SELECT * FROM a {join_syntax} b")
        else:
            tests.append(f"SELECT * FROM a {join_syntax} b ON a.id = b.id")
        tests.append("")
    
    return "\n".join(tests)


def generate_expression_tests() -> str:
    """Generate test SQL for expression types."""
    tests = []
    tests.append("-- Grammar-generated expression tests")
    tests.append("")
    
    # Lambda expressions
    tests.append("-- Lambda expressions")
    tests.append("SELECT transform(arr, x -> x + 1) FROM t")
    tests.append("SELECT filter(arr, x -> x > 0) FROM t")
    tests.append("SELECT transform(arr, (k, v) -> k + v) FROM t")
    tests.append("SELECT aggregate(arr, 0, (acc, x) -> acc + x) FROM t")
    tests.append("")
    
    # Type casts
    tests.append("-- Type casts")
    tests.append("SELECT CAST(x AS INT) FROM t")
    tests.append("SELECT CAST(x AS STRING) FROM t")
    tests.append("SELECT CAST(x AS DECIMAL(10, 2)) FROM t")
    tests.append("SELECT TRY_CAST(x AS INT) FROM t")
    tests.append("SELECT x::INT FROM t")
    tests.append("SELECT x::DECIMAL(10, 2) FROM t")
    tests.append("")
    
    # Semi-structured access
    tests.append("-- Semi-structured field access")
    tests.append("SELECT col:field FROM t")
    tests.append("SELECT col:field.subfield FROM t")
    tests.append("SELECT col:field['key'] FROM t")
    tests.append("SELECT col:array[0] FROM t")
    tests.append("")
    
    # Named arguments
    tests.append("-- Named arguments in function calls")
    tests.append("SELECT func(param => value) FROM t")
    tests.append("SELECT func(a, b => 1, c => 2) FROM t")
    tests.append("")
    
    # Nested field access
    tests.append("-- Nested struct field access")
    tests.append("SELECT a.b FROM t")
    tests.append("SELECT a.b.c FROM t")
    tests.append("SELECT a.b.c.d FROM t")
    tests.append("SELECT a.b.c.d.e FROM t")
    tests.append("")
    
    return "\n".join(tests)


def generate_string_tests() -> str:
    """Generate test SQL for string handling."""
    tests = []
    tests.append("-- Grammar-generated string and identifier tests")
    tests.append("")
    
    # String literals
    tests.append("-- Single-quoted strings")
    tests.append("SELECT 'hello' FROM t")
    tests.append("SELECT 'hello''s' FROM t")  # Escaped quote
    tests.append("SELECT '' FROM t")  # Empty string
    tests.append("")
    
    # Quoted identifiers
    tests.append("-- Backtick-quoted identifiers")
    tests.append("SELECT `column` FROM t")
    tests.append("SELECT `my-column` FROM t")
    tests.append("SELECT `my column` FROM t")
    tests.append("SELECT `select` FROM t")  # Keyword as identifier
    tests.append("SELECT `FROM` FROM t")  # Keyword as identifier
    tests.append("")
    
    tests.append("-- Double-quoted identifiers (when enabled)")
    tests.append('SELECT "column" FROM t')
    tests.append('SELECT "my-column" FROM t')
    tests.append("")
    
    return "\n".join(tests)


def find_gaps(grammar_keywords: List[str], current_keywords: Set[str]) -> Tuple[List[str], List[str]]:
    """Find keywords in grammar but not in implementation, and vice versa."""
    grammar_set = set(kw.upper() for kw in grammar_keywords)
    
    missing_from_impl = sorted(grammar_set - current_keywords)
    extra_in_impl = sorted(current_keywords - grammar_set)
    
    return missing_from_impl, extra_in_impl


def main():
    print("=== Spark SQL Grammar Analysis ===\n")
    
    # Download grammar files
    try:
        lexer_content = download_grammar(LEXER_URL)
        # parser_content = download_grammar(PARSER_URL)  # Not needed for basic analysis
    except Exception as e:
        print(f"Error downloading grammar: {e}")
        print("Using local analysis only...")
        lexer_content = ""
    
    # Extract grammar elements
    keywords = extract_keywords_from_lexer(lexer_content) if lexer_content else []
    operators = extract_operators_from_lexer(lexer_content) if lexer_content else {}
    literals = extract_literal_patterns(lexer_content)
    
    print(f"Found {len(keywords)} keywords in grammar")
    print(f"Found {len(operators)} operators in grammar")
    
    # Read current implementation
    current_keywords = read_current_keywords(PROJECT_ROOT)
    print(f"Found {len(current_keywords)} keywords in current implementation")
    
    # Find gaps
    if keywords:
        missing, extra = find_gaps(keywords, current_keywords)
        print(f"\nMissing from implementation: {len(missing)} keywords")
        if missing:
            print("  " + ", ".join(missing[:20]) + ("..." if len(missing) > 20 else ""))
        print(f"Extra in implementation: {len(extra)} keywords")
        if extra:
            print("  " + ", ".join(extra[:20]) + ("..." if len(extra) > 20 else ""))
    
    # Create test data directory
    TEST_DATA_DIR.mkdir(parents=True, exist_ok=True)
    
    # Generate test files
    test_files = [
        ("01_keywords.sql", generate_keyword_tests(keywords)),
        ("02_operators.sql", generate_operator_tests(operators)),
        ("03_literals.sql", generate_literal_tests(literals)),
        ("04_clauses.sql", generate_clause_tests()),
        ("05_joins.sql", generate_join_tests()),
        ("06_expressions.sql", generate_expression_tests()),
        ("07_strings.sql", generate_string_tests()),
    ]
    
    for filename, content in test_files:
        filepath = TEST_DATA_DIR / filename
        filepath.write_text(content)
        print(f"Generated: {filepath}")
    
    # Save analysis results as JSON
    analysis = {
        "keywords": keywords,
        "operators": operators,
        "literals": literals,
        "missing_keywords": missing if keywords else [],
        "extra_keywords": extra if keywords else [],
    }
    
    OUTPUT_JSON.write_text(json.dumps(analysis, indent=2))
    print(f"\nAnalysis saved to: {OUTPUT_JSON}")
    
    print("\n=== Summary ===")
    print(f"Total grammar keywords: {len(keywords)}")
    print(f"Implementation keywords: {len(current_keywords)}")
    if keywords:
        print(f"Coverage: {len(current_keywords & set(kw.upper() for kw in keywords))}/{len(keywords)} ({100*len(current_keywords & set(kw.upper() for kw in keywords))//len(keywords)}%)")


if __name__ == "__main__":
    main()
