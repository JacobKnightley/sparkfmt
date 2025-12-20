# MANDATORY: Grammar-Driven Architecture Implementation

## Executive Mandate

> **This project MUST be grammar-driven, NOT rule/linter-driven.**
>
> The Apache Spark ANTLR grammar files (`SqlBaseLexer.g4`, `SqlBaseParser.g4`) are the **single source of truth**. All keywords, operators, tokens, and syntactic constructs must be derived from these files—not hand-coded.
>
> **Non-negotiable requirement:** If Spark's grammar supports it, we support it. No exceptions. No silent drops. No "we'll add that later."

---

## Background: Why Grammar-Driven?

Traditional SQL formatters are "linter-driven"—developers manually enumerate keywords, operators, and rules based on documentation or experience. This approach:

1. **Drifts from reality** - New Spark versions add keywords; hand-coded lists fall behind
2. **Has blind spots** - Obscure syntax gets missed (`<=>`, `::`, `->`, etc.)
3. **Causes silent data loss** - Unknown constructs get dropped or corrupted
4. **Requires constant maintenance** - Every Spark release needs manual review

A **grammar-driven** formatter:

1. **Provably complete** - Derived from the same grammar Spark uses
2. **Auto-updates** - New keywords/syntax automatically included
3. **Fails loudly** - Unknown constructs cause build/test failures, not silent corruption
4. **Self-documenting** - Grammar IS the specification

---

## Architecture Requirements

### 1. Grammar Files as Source of Truth

```
grammar/
  SqlBaseLexer.g4      # Downloaded from Apache Spark repo
  SqlBaseParser.g4     # Downloaded from Apache Spark repo
  VERSION              # Tracks which Spark version/commit
```

**Source URLs:**
- Lexer: `https://raw.githubusercontent.com/apache/spark/master/sql/api/src/main/antlr4/org/apache/spark/sql/catalyst/parser/SqlBaseLexer.g4`
- Parser: `https://raw.githubusercontent.com/apache/spark/master/sql/api/src/main/antlr4/org/apache/spark/sql/catalyst/parser/SqlBaseParser.g4`

These files MUST be committed to the repo and kept in sync with Spark releases.

---

### 2. Build-Time Code Generation

Create `build.rs` that:

1. **Parses `SqlBaseLexer.g4`** to extract:
   - All keywords (between `//--SPARK-KEYWORD-LIST-START` and `//--SPARK-KEYWORD-LIST-END`)
   - All operators with their symbols
   - All literal patterns (numbers, strings, identifiers)

2. **Parses `SqlBaseParser.g4`** to extract:
   - Grammar rule names and structures
   - Clause ordering (e.g., `queryOrganization` defines ORDER BY, CLUSTER BY, etc.)
   - Expression precedence

3. **Generates Rust code** into `src/generated/`:

```rust
// src/generated/keywords.rs - AUTO-GENERATED, DO NOT EDIT
pub const SPARK_KEYWORDS: &[&str] = &[
    "ADD", "AFTER", "AGGREGATE", "ALL", "ALTER", "ALWAYS", "ANALYZE",
    // ... all 399 keywords from grammar
];

pub fn is_keyword(s: &str) -> bool {
    SPARK_KEYWORDS.binary_search(&s.to_uppercase().as_str()).is_ok()
}
```

```rust
// src/generated/tokens.rs - AUTO-GENERATED, DO NOT EDIT
pub enum Operator {
    Eq,           // =, ==
    NullSafeEq,   // <=>
    NotEq,        // <>, !=
    Lt, Lte, Gt, Gte,
    ShiftLeft,    // <<
    ShiftRight,   // >>
    Arrow,        // ->
    FatArrow,     // =>
    DoubleColon,  // ::
    ConcatPipe,   // ||
    // ... all operators from grammar
}

pub const OPERATOR_SYMBOLS: &[(&str, Operator)] = &[
    ("<=>", Operator::NullSafeEq),
    ("!=", Operator::NotEq),
    ("<>", Operator::NotEq),
    // ... ordered by length (longest first for lexer matching)
];
```

```rust
// src/generated/rules.rs - AUTO-GENERATED, DO NOT EDIT
pub enum GrammarRule {
    // From queryOrganization
    OrderByClause,
    ClusterByClause,
    DistributeByClause,
    SortByClause,
    LimitClause,
    OffsetClause,
    
    // From queryTerm
    UnionOperation,
    ExceptOperation,
    IntersectOperation,
    MinusOperation,
    
    // From joinRelation
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullJoin,
    CrossJoin,
    LeftSemiJoin,
    LeftAntiJoin,
    NaturalJoin,
    
    // From primaryExpression
    LambdaExpression,
    CastExpression,
    CastByColon,
    // ... all grammar rules
}
```

---

### 3. Parser Must Use Generated Definitions

The parser MUST NOT have hand-coded keyword lists. Instead:

```rust
// WRONG - Hand-coded, will drift
fn is_keyword(s: &str) -> bool {
    matches!(s.to_uppercase().as_str(), 
        "SELECT" | "FROM" | "WHERE" | /* incomplete list */ )
}

// CORRECT - Uses generated code
use crate::generated::keywords;

fn is_keyword(s: &str) -> bool {
    keywords::is_keyword(s)
}
```

---

### 4. Lexer Must Handle All Token Types

From `SqlBaseLexer.g4`, the lexer MUST correctly tokenize:

#### Number Literals
| Pattern | Example | ANTLR Rule |
|---------|---------|------------|
| Integer | `123` | `INTEGER_VALUE` |
| Decimal | `123.456` | `DECIMAL_VALUE` |
| Scientific | `1.5e10`, `1E-5` | `EXPONENT_VALUE` |
| BigInt | `100L` | `BIGINT_LITERAL` |
| SmallInt | `50S` | `SMALLINT_LITERAL` |
| TinyInt | `10Y` | `TINYINT_LITERAL` |
| Float | `3.14F` | `FLOAT_LITERAL` |
| Double | `2.718D` | `DOUBLE_LITERAL` |
| BigDecimal | `99.99BD` | `BIGDECIMAL_LITERAL` |

#### String/Identifier Literals
| Pattern | Example | ANTLR Rule |
|---------|---------|------------|
| Single-quoted | `'hello'` | `STRING_LITERAL` |
| Double-quoted | `"column"` | `DOUBLEQUOTED_STRING` |
| Backtick | `` `my column` `` | `BACKQUOTED_IDENTIFIER` |
| Hex binary | `X'1F2A'` | `BINARY_HEX` |

#### Operators (Ordered by Length for Lexing)
```
<=>   !=   <>   <=   >=   <<   >>   >>>   ||   |>   ::   ->   =>
=     <    >    +    -    *    /    %    &    |    ^    ~    :
```

---

### 5. All Grammar Rules Must Be Handled

The parser MUST recognize all constructs from `SqlBaseParser.g4`:

#### Query Organization (from `queryOrganization`)
```sql
SELECT * FROM t
    ORDER BY x              -- MUST support
    CLUSTER BY y            -- MUST support (Spark-specific)
    DISTRIBUTE BY z         -- MUST support (Spark-specific)
    SORT BY w               -- MUST support (Spark-specific)
    LIMIT 10                -- MUST support
    OFFSET 5                -- MUST support
```

#### Set Operations (from `queryTerm`)
```sql
SELECT * FROM a UNION SELECT * FROM b           -- MUST support
SELECT * FROM a UNION ALL SELECT * FROM b       -- MUST support
SELECT * FROM a EXCEPT SELECT * FROM b          -- MUST support
SELECT * FROM a INTERSECT SELECT * FROM b       -- MUST support
SELECT * FROM a MINUS SELECT * FROM b           -- MUST support (Spark alias)
```

#### Join Types (from `joinType`)
```sql
JOIN                    -- MUST support
INNER JOIN              -- MUST support
LEFT JOIN               -- MUST support
LEFT OUTER JOIN         -- MUST support
RIGHT JOIN              -- MUST support
RIGHT OUTER JOIN        -- MUST support
FULL JOIN               -- MUST support
FULL OUTER JOIN         -- MUST support
CROSS JOIN              -- MUST support
LEFT SEMI JOIN          -- MUST support (Spark-specific)
LEFT ANTI JOIN          -- MUST support (Spark-specific)
NATURAL JOIN            -- MUST support
NATURAL LEFT JOIN       -- MUST support
```

#### Expressions (from `primaryExpression`)
```sql
x -> x + 1                      -- Lambda (MUST support)
(x, y) -> x + y                 -- Multi-param lambda (MUST support)
CAST(x AS INT)                  -- Cast (MUST support)
TRY_CAST(x AS INT)              -- Try cast (MUST support)
x::INT                          -- Double-colon cast (MUST support)
col:field                       -- Semi-structured access (MUST support)
col:field.subfield              -- Nested semi-structured (MUST support)
func(param => value)            -- Named arguments (MUST support)
a.b.c.d.e                       -- Unlimited nested field access (MUST support)
```

---

### 6. Failure Mode: Loud, Not Silent

**CRITICAL:** Unknown tokens/constructs MUST cause failures, not silent drops.

```rust
// FORBIDDEN - Silent drop causes data loss
match token {
    ParserToken::Word(w) => { /* handle */ }
    ParserToken::Number(n) => { /* handle */ }
    _ => {}  // SILENT DROP - NEVER DO THIS
}

// REQUIRED - Fail loudly or preserve
match token {
    ParserToken::Word(w) => { /* handle */ }
    ParserToken::Number(n) => { /* handle */ }
    other => {
        return Err(FormatError::new(format!(
            "Unhandled token type: {:?}. This indicates a gap in grammar coverage.",
            other
        )));
    }
}
```

---

### 7. Completeness Verification

Build MUST include verification that all grammar constructs are handled:

```rust
// build.rs or test
fn verify_grammar_coverage() {
    let grammar_keywords = parse_grammar_keywords();
    let implemented_keywords = get_implemented_keywords();
    
    let missing: Vec<_> = grammar_keywords
        .difference(&implemented_keywords)
        .collect();
    
    if !missing.is_empty() {
        panic!(
            "GRAMMAR COVERAGE FAILURE: Missing {} keywords: {:?}",
            missing.len(),
            missing
        );
    }
}
```

---

## Implementation Steps

### Step 1: Add Grammar Files

```bash
mkdir -p grammar
curl -o grammar/SqlBaseLexer.g4 "https://raw.githubusercontent.com/apache/spark/master/sql/api/src/main/antlr4/org/apache/spark/sql/catalyst/parser/SqlBaseLexer.g4"
curl -o grammar/SqlBaseParser.g4 "https://raw.githubusercontent.com/apache/spark/master/sql/api/src/main/antlr4/org/apache/spark/sql/catalyst/parser/SqlBaseParser.g4"
echo "spark-master-$(date +%Y%m%d)" > grammar/VERSION
```

### Step 2: Create build.rs

Create `crates/sparkfmt-core/build.rs` that:
1. Reads `grammar/SqlBaseLexer.g4`
2. Extracts keywords between `//--SPARK-KEYWORD-LIST-START` and `//--SPARK-KEYWORD-LIST-END`
3. Extracts operator definitions
4. Generates `src/generated/keywords.rs`, `tokens.rs`, `rules.rs`

### Step 3: Replace Hand-Coded Keywords

Delete the current hand-coded `keywords.rs` and replace with generated version.

### Step 4: Update Lexer

Lexer must handle all token types from grammar:
- Scientific notation (`EXPONENT_VALUE`)
- Suffixed literals (`L`, `S`, `Y`, `F`, `D`, `BD`)
- All operators (especially multi-char: `<=>`, `::`, `->`, `=>`, `||`, `|>`)
- All string/identifier formats

### Step 5: Update Parser

Parser must recognize all grammar rules:
- `queryOrganization`: CLUSTER BY, DISTRIBUTE BY, SORT BY
- `queryTerm`: EXCEPT, INTERSECT, MINUS
- `primaryExpression`: Lambda, double-colon cast, semi-structured access
- Unlimited nesting for field access

### Step 6: Add Coverage Tests

Generate tests from grammar that verify every keyword, operator, and construct works.

---

## Verification Criteria

The implementation is complete when:

1. **`cargo build` generates** `keywords.rs`, `tokens.rs`, `rules.rs` from grammar files
2. **No hand-coded keyword lists** exist anywhere in the codebase
3. **All 399 grammar keywords** are recognized
4. **All operators** are correctly lexed (including `<=>`, `::`, `->`, etc.)
5. **All number formats** are preserved (scientific notation, suffixed literals)
6. **All grammar constructs** produce valid formatted output
7. **Build fails** if grammar coverage is incomplete
8. **Tests verify** every grammar construct round-trips correctly

---

## Non-Negotiable Constraints

1. **Grammar is truth** - If there's a conflict between grammar and implementation, grammar wins
2. **No silent drops** - Every token must be handled explicitly
3. **Auto-generation** - Keywords and tokens come from build.rs, not manual coding
4. **Completeness** - Partial support is a bug; full grammar support is the requirement
5. **Loud failures** - Unknown constructs fail the build or return errors, never silently corrupt

---

## Reference: Current Gaps to Fix

Based on grammar analysis, these are currently broken/missing:

| Category | Issue | Grammar Source |
|----------|-------|----------------|
| Keywords | 165 missing keywords | `SqlBaseLexer.g4` SPARK-KEYWORD-LIST |
| Operators | `<=>`, `::`, `->`, `=>`, `\|\|`, `\|>` | `SqlBaseLexer.g4` operator definitions |
| Numbers | Scientific notation (e.g., `1.5e10`) | `EXPONENT_VALUE` rule |
| Numbers | Suffixed literals (`100L`, `50S`, etc.) | `*_LITERAL` rules |
| Clauses | CLUSTER BY, DISTRIBUTE BY, SORT BY | `queryOrganization` rule |
| Set Ops | EXCEPT, INTERSECT, MINUS | `queryTerm` rule |
| Expressions | Lambda (`x -> x+1`) | `primaryExpression` #lambda |
| Expressions | Double-colon cast (`x::INT`) | `primaryExpression` #castByColon |
| Expressions | Nested field access (>2 levels) | `primaryExpression` #dereference |

---

## Summary

This formatter must be **derived from grammar**, not **approximating grammar**. The ANTLR files are the specification. Our implementation must be provably aligned with that specification through:

1. Build-time code generation from grammar files
2. Compile-time/test-time verification of complete coverage
3. Loud failures for any unhandled constructs

**If Spark can parse it, we can format it. No exceptions.**
