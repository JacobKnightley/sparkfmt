# Grammar-Driven Parser Refactor

## Objective

Replace the hand-written parser with an ANTLR-generated parser that uses Apache Spark's official grammar files as the single source of truth. When complete, there should be **zero hard-coded keywords, operators, or syntax rules** - everything derives from the grammar.

## Current State

The foundation is in place:
- `build_antlr.py` - Downloads grammar, transforms for Rust, generates ANTLR code, post-processes
- `antlr_predicates.rs` - Extension traits implementing all semantic predicates
- `src/generated/` - ANTLR-generated lexer and parser (compiles successfully)
- `KNOWN_PREDICATES.json` - Catalog of all 16 predicates for change detection

The generated ANTLR parser **compiles** but is **not integrated**. The formatter still uses the old hand-written parser.

## Tasks to Complete

### 1. Create ANTLR Parser Wrapper

Create `src/antlr_parser.rs` that wraps the generated ANTLR parser:

```rust
// Wrap the generated parser to provide a clean API
pub fn parse(input: &str) -> Result<ParseTree, FormatError> {
    // 1. Create input stream from string
    // 2. Create lexer with LexerPredicates trait in scope
    // 3. Create token stream
    // 4. Create parser with ParserPredicates trait in scope
    // 5. Call parser.singleStatement() or parser.query()
    // 6. Return parse tree or convert errors
}
```

Reference the antlr4rust crate documentation and examples for the exact API.

### 2. Create AST Converter

Create `src/ast_converter.rs` that walks the ANTLR parse tree and builds our IR:

The ANTLR parser produces a concrete syntax tree (CST) with context nodes like:
- `QueryContext`
- `SelectClauseContext`
- `FromClauseContext`
- `WhereClauseContext`
- etc.

Convert these to our existing IR types in `src/ir.rs`:
- `Query`
- `SelectClause`
- `FromClause`
- `WhereClause`
- etc.

Use the ANTLR listener or visitor pattern:
```rust
impl SqlBaseParserListener for AstBuilder {
    fn enter_selectClause(&mut self, ctx: &SelectClauseContext) {
        // Build SelectClause IR node
    }
    // ... etc
}
```

### 3. Remove Hard-Coded Lists

Delete or replace these with grammar-derived sources:

| Current Hard-Coding | Replace With |
|---------------------|--------------|
| `keywords.rs` - manually curated keyword list | Parse from ANTLR token types |
| `functions.rs` - hard-coded function names | Remove - functions are just identifiers followed by `(` |
| `build.rs` keyword extraction | Remove - ANTLR tokens are the source |
| `build_generated.rs` | Remove once parser.rs stops using it |
| `parser.rs` - the entire hand-written parser | Remove once ANTLR parser is working |

### 4. Wire Up the Formatter

Update `src/formatter.rs` to use the new ANTLR-based parser:

```rust
pub fn format(input: &str) -> Result<String, FormatError> {
    // 1. Parse with ANTLR parser
    let tree = antlr_parser::parse(input)?;
    
    // 2. Convert to IR
    let query = ast_converter::convert(tree)?;
    
    // 3. Format IR to string (existing logic)
    let output = print_query(&query);
    
    Ok(output)
}
```

### 5. Handle Comments

ANTLR puts comments in a separate channel. Implement comment extraction and anchoring:

```rust
// During lexing, collect all comment tokens
// Attach each comment to nearest syntax node based on line/column
// Preserve comment placement during formatting
```

### 6. Update Tests

- All existing tests should pass with the new parser
- Add tests for grammar edge cases that the hand-written parser missed
- The 3 failing tests (CLUSTER BY, DISTRIBUTE BY, SORT BY) should now pass since ANTLR handles them

### 7. Clean Up

Once everything works:
- Delete `src/parser.rs` (hand-written parser)
- Delete `src/keywords.rs` (hard-coded keywords)
- Delete `src/functions.rs` (hard-coded functions)
- Delete `src/build_generated.rs` (build.rs output wrapper)
- Simplify `build.rs` (may not need keyword extraction anymore)
- Update `lib.rs` exports

## Architecture After Refactor

```
Input SQL String
       ↓
ANTLR Lexer (generated, uses LexerPredicates trait)
       ↓
Token Stream (with comments in separate channel)
       ↓
ANTLR Parser (generated, uses ParserPredicates trait)
       ↓
Concrete Syntax Tree (ANTLR contexts)
       ↓
AST Converter (ast_converter.rs)
       ↓
IR (ir.rs - Query, SelectClause, etc.)
       ↓
Formatter (formatter.rs - existing print logic)
       ↓
Output SQL String
```

## Key Files Reference

| File | Purpose |
|------|---------|
| `src/generated/sqlbaselexer.rs` | ANTLR-generated lexer |
| `src/generated/sqlbaseparser.rs` | ANTLR-generated parser |
| `src/generated/sqlbaseparserlistener.rs` | Listener interface for tree walking |
| `src/antlr_predicates.rs` | Predicate implementations (LexerPredicates, ParserPredicates traits) |
| `src/antlr4rust_workarounds.rs` | Re-exports traits for generated code scope |
| `src/ir.rs` | IR types (keep as-is, this is our formatting model) |
| `src/formatter.rs` | Formatting logic (keep as-is, just change input source) |

## Success Criteria

1. **All tests pass** (including the 3 currently failing Spark clause tests)
2. **No hard-coded keyword/operator lists** - everything from grammar
3. **Grammar updates are automatic** - run `build_antlr.py all`, rebuild, done
4. **New predicates cause build failure** - analyze step detects unknown predicates
5. **Comments preserved** - same behavior as current formatter
6. **Performance acceptable** - no major regression in format speed

## Non-Goals

- Modifying the grammar files (we use Spark's grammar as-is)
- Supporting multiple SQL dialects (Spark SQL only)
- Pretty-printing the parse tree (we have our own IR for that)
