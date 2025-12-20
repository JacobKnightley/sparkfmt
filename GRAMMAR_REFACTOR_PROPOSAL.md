# MANDATORY: Grammar-Driven Parser Refactor

> **THIS IS A NON-NEGOTIABLE REFACTOR. DO NOT DEVIATE FROM THIS PLAN.**

## Executive Mandate

Replace the hand-written parser with the ANTLR-generated parser. The Apache Spark grammar files (`SqlBaseLexer.g4`, `SqlBaseParser.g4`) are the **SINGLE SOURCE OF TRUTH**.

**When complete:**
- ZERO hand-coded keywords
- ZERO hand-coded operators  
- ZERO hand-coded syntax rules
- EVERYTHING derives from grammar

**Non-negotiable requirement:** If Spark's grammar supports it, we support it. No exceptions. No silent drops.

---

## Current State (Foundation Complete)

These files exist and work:

| File | Status | Purpose |
|------|--------|---------|
| `grammar/SqlBaseLexer.g4` | ✅ DONE | Source grammar (lexer) |
| `grammar/SqlBaseParser.g4` | ✅ DONE | Source grammar (parser) |
| `build_antlr.py` | ✅ DONE | Downloads, transforms, generates, post-processes |
| `src/antlr_predicates.rs` | ✅ DONE | All 16 predicate implementations |
| `src/antlr4rust_workarounds.rs` | ✅ DONE | Trait re-exports for generated code |
| `src/generated/` | ✅ DONE | ANTLR-generated lexer/parser (compiles) |
| `KNOWN_PREDICATES.json` | ✅ DONE | Predicate catalog for change detection |

**The generated ANTLR parser COMPILES but is NOT INTEGRATED.**

---

## TASK 1: Create ANTLR Parser Wrapper

**File:** `src/antlr_parser.rs`

**REQUIRED Implementation:**

```rust
//! ANTLR Parser Wrapper
//! 
//! This module wraps the generated ANTLR parser to provide a clean API.
//! It is the ONLY entry point for parsing SQL strings.

use crate::error::FormatError;
use crate::generated::sqlbaselexer::SqlBaseLexer;
use crate::generated::sqlbaseparser::SqlBaseParser;
use crate::antlr_predicates::{LexerPredicates, ParserPredicates, reset_lexer_state};
use antlr4rust::input_stream::InputStream;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::tree::ParseTree;

/// Parse SQL string into ANTLR parse tree.
/// 
/// MUST handle all valid Spark SQL syntax.
/// MUST return error for invalid syntax (never panic).
/// MUST reset lexer state before parsing.
pub fn parse(input: &str) -> Result<Box<dyn ParseTree>, FormatError> {
    // Reset thread-local state
    reset_lexer_state();
    
    // Create input stream
    let input_stream = InputStream::new(input);
    
    // Create lexer (LexerPredicates trait in scope via antlr4rust_workarounds)
    let lexer = SqlBaseLexer::new(input_stream);
    
    // Create token stream
    let token_stream = CommonTokenStream::new(lexer);
    
    // Create parser (ParserPredicates trait in scope via antlr4rust_workarounds)
    let mut parser = SqlBaseParser::new(token_stream);
    
    // Parse - use singleStatement for complete SQL statements
    let tree = parser.singleStatement()
        .map_err(|e| FormatError::new(format!("Parse error: {:?}", e)))?;
    
    Ok(tree)
}

/// Extract comments from token stream.
/// 
/// ANTLR puts comments in channel 2 (HIDDEN).
/// MUST collect all comments with their positions.
pub fn extract_comments(token_stream: &CommonTokenStream) -> Vec<Comment> {
    // Iterate all tokens including hidden channel
    // Filter for COMMENT and BRACKETED_COMMENT token types
    // Return with line/column positions
    todo!("Implement comment extraction")
}
```

**CONSTRAINTS:**
- MUST use `reset_lexer_state()` before each parse
- MUST NOT panic on invalid input
- MUST return `FormatError` for parse failures
- MUST extract comments from hidden channel

---

## TASK 2: Create AST Converter

**File:** `src/ast_converter.rs`

**Purpose:** Convert ANTLR Concrete Syntax Tree (CST) to our IR types.

**REQUIRED Implementation:**

```rust
//! AST Converter
//!
//! Converts ANTLR parse tree to our formatting IR.
//! MUST handle ALL grammar rules - no silent drops.

use crate::error::FormatError;
use crate::ir::*;
use crate::generated::sqlbaseparser::*;
use crate::generated::sqlbaseparserlistener::SqlBaseParserListener;

/// Convert ANTLR parse tree to IR Query.
///
/// MUST handle every node type from the grammar.
/// MUST return error for unhandled nodes (NEVER silent drop).
pub fn convert(tree: &dyn ParseTree) -> Result<Query, FormatError> {
    let mut builder = AstBuilder::new();
    // Walk tree and build IR
    todo!("Implement tree walking")
}

struct AstBuilder {
    // Stack-based building of nested structures
}

impl AstBuilder {
    fn new() -> Self {
        Self { /* ... */ }
    }
}

impl SqlBaseParserListener for AstBuilder {
    // MUST implement handlers for ALL context types:
    
    fn enter_selectClause(&mut self, ctx: &SelectClauseContext) {
        // Build SelectClause
    }
    
    fn enter_fromClause(&mut self, ctx: &FromClauseContext) {
        // Build FromClause
    }
    
    fn enter_whereClause(&mut self, ctx: &WhereClauseContext) {
        // Build WhereClause
    }
    
    fn enter_queryOrganization(&mut self, ctx: &QueryOrganizationContext) {
        // MUST handle: ORDER BY, CLUSTER BY, DISTRIBUTE BY, SORT BY, LIMIT, OFFSET
    }
    
    fn enter_queryTerm(&mut self, ctx: &QueryTermContext) {
        // MUST handle: UNION, EXCEPT, INTERSECT, MINUS
    }
    
    fn enter_joinRelation(&mut self, ctx: &JoinRelationContext) {
        // MUST handle: INNER, LEFT, RIGHT, FULL, CROSS, LEFT SEMI, LEFT ANTI, NATURAL
    }
    
    fn enter_primaryExpression(&mut self, ctx: &PrimaryExpressionContext) {
        // MUST handle: Lambda, Cast, CastByColon, FieldAccess (unlimited depth)
    }
    
    // ... ALL other context handlers
}
```

**CRITICAL CONSTRAINT - NO SILENT DROPS:**

```rust
// FORBIDDEN - This causes data loss
fn handle_unknown(&mut self, ctx: &dyn Context) {
    // silently ignore
}

// REQUIRED - Fail loudly
fn handle_unknown(&mut self, ctx: &dyn Context) -> Result<(), FormatError> {
    Err(FormatError::new(format!(
        "UNHANDLED GRAMMAR CONSTRUCT: {}. This is a coverage gap that MUST be fixed.",
        ctx.get_rule_name()
    )))
}
```

**Grammar Constructs That MUST Be Handled:**

| Grammar Rule | Constructs | Current Status |
|--------------|------------|----------------|
| `queryOrganization` | ORDER BY, CLUSTER BY, DISTRIBUTE BY, SORT BY, LIMIT, OFFSET | CLUSTER/DISTRIBUTE/SORT BY failing |
| `queryTerm` | UNION, UNION ALL, EXCEPT, INTERSECT, MINUS | Needs verification |
| `joinType` | INNER, LEFT, RIGHT, FULL, CROSS, LEFT SEMI, LEFT ANTI, NATURAL | Needs verification |
| `primaryExpression` | Lambda (`x -> x+1`), CastByColon (`x::INT`), unlimited field access | Currently broken |
| `number` | Scientific notation, suffixed literals (L, S, Y, F, D, BD) | Currently broken |

---

## TASK 3: Delete Hand-Coded Files

**MANDATORY DELETIONS after Tasks 1-2 are complete:**

| File | Action | Reason |
|------|--------|--------|
| `src/parser.rs` | **DELETE ENTIRELY** | Replaced by ANTLR parser |
| `src/keywords.rs` | **DELETE ENTIRELY** | Keywords come from ANTLR tokens |
| `src/functions.rs` | **DELETE ENTIRELY** | Functions are identifiers + `(` |
| `src/build_generated.rs` | **DELETE ENTIRELY** | No longer needed |

**MODIFY `build.rs`:**
- REMOVE all keyword extraction logic
- KEEP only if needed for non-ANTLR generation

**FORBIDDEN:**
- DO NOT keep "backup" copies of deleted files
- DO NOT comment out code instead of deleting
- DO NOT create new hand-coded lists

---

## TASK 4: Wire Up Formatter

**File:** `src/formatter.rs`

**REQUIRED Changes:**

```rust
// BEFORE (hand-written parser)
pub fn format(input: &str) -> Result<String, FormatError> {
    let tokens = lexer::tokenize(input)?;
    let query = parser::parse(&tokens)?;  // DELETE THIS
    let output = print_query(&query);
    Ok(output)
}

// AFTER (ANTLR parser)
pub fn format(input: &str) -> Result<String, FormatError> {
    // 1. Parse with ANTLR
    let tree = antlr_parser::parse(input)?;
    
    // 2. Extract comments from token stream
    let comments = antlr_parser::extract_comments(&tree.token_stream)?;
    
    // 3. Convert CST to IR
    let mut query = ast_converter::convert(&tree)?;
    
    // 4. Attach comments to IR nodes
    attach_comments(&mut query, comments)?;
    
    // 5. Format IR to string (existing logic - keep as-is)
    let output = print_query(&query);
    
    Ok(output)
}
```

**CONSTRAINTS:**
- MUST NOT modify `print_query` or IR printing logic
- MUST preserve existing formatting rules
- MUST maintain comment anchoring behavior

---

## TASK 5: Comment Handling

**REQUIRED Implementation:**

```rust
/// Comment with position and attachment info.
pub struct Comment {
    pub text: String,
    pub line: usize,
    pub column: usize,
    pub is_block: bool,  // true for /* */, false for --
    pub is_hint: bool,   // true for /*+ */
}

/// Attach comments to nearest IR nodes.
///
/// Rules (from copilot-instructions.md):
/// 1. TrailingInline: Line comment after code on same line -> attach to preceding token
/// 2. TrailingOwnLine: Comment on own line -> print above attached node
/// 3. Leading: Comment before node -> print before node
///
/// MUST NOT:
/// - Move comments across clause boundaries
/// - Inline block comments into expressions
/// - Drop any comments
pub fn attach_comments(query: &mut Query, comments: Vec<Comment>) -> Result<(), FormatError> {
    for comment in comments {
        let anchor = find_anchor(query, &comment)?;
        anchor.attach_comment(comment);
    }
    Ok(())
}
```

---

## TASK 6: Update lib.rs

**REQUIRED Changes:**

```rust
// ADD these modules
pub mod antlr_parser;
pub mod ast_converter;

// REMOVE these modules (after deletion)
// pub mod parser;      // DELETED
// pub mod keywords;    // DELETED  
// pub mod functions;   // DELETED
// pub mod build_generated; // DELETED

// KEEP these modules
pub mod antlr_predicates;
pub mod antlr4rust_workarounds;
pub mod error;
pub mod formatter;
pub mod hints;
pub mod ir;
pub mod generated;
```

---

## TASK 7: Fix All Tests

**REQUIRED Outcomes:**

| Test Category | Current | Required |
|---------------|---------|----------|
| Basic SELECT | ✅ Pass | ✅ Pass |
| JOINs | ✅ Pass | ✅ Pass |
| Subqueries | ✅ Pass | ✅ Pass |
| CTEs | ✅ Pass | ✅ Pass |
| Comments | ✅ Pass | ✅ Pass |
| CLUSTER BY | ❌ Fail | ✅ **MUST PASS** |
| DISTRIBUTE BY | ❌ Fail | ✅ **MUST PASS** |
| SORT BY | ❌ Fail | ✅ **MUST PASS** |
| Scientific notation | ❌ Broken | ✅ **MUST PASS** |
| Nested field access | ❌ Broken | ✅ **MUST PASS** |
| Lambda expressions | ❌ Broken | ✅ **MUST PASS** |
| Double-colon cast | ❌ Broken | ✅ **MUST PASS** |

**ADD New Tests For:**
- All set operations: UNION, UNION ALL, EXCEPT, INTERSECT, MINUS
- All join types: LEFT SEMI, LEFT ANTI, NATURAL variants
- All number formats: `1.5e10`, `100L`, `50S`, `10Y`, `3.14F`, `2.718D`, `99.99BD`
- All operators: `<=>`, `::`, `->`, `=>`, `||`, `|>`

---

## Architecture After Refactor

```
Input SQL String
       │
       ▼
┌──────────────────────────────────────┐
│  ANTLR Lexer (generated)             │
│  - Uses LexerPredicates trait        │
│  - Comments to hidden channel        │
└──────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────┐
│  ANTLR Parser (generated)            │
│  - Uses ParserPredicates trait       │
│  - Full Spark SQL grammar coverage   │
└──────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────┐
│  AST Converter (ast_converter.rs)    │
│  - CST → IR transformation           │
│  - NO silent drops                   │
└──────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────┐
│  IR (ir.rs)                          │
│  - Query, SelectClause, etc.         │
│  - Comments attached to nodes        │
└──────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────┐
│  Formatter (formatter.rs)            │
│  - Existing print logic              │
│  - No changes needed                 │
└──────────────────────────────────────┘
       │
       ▼
Output SQL String
```

---

## Files Reference

**KEEP (No Changes):**
| File | Purpose |
|------|---------|
| `src/ir.rs` | IR types - formatting model |
| `src/error.rs` | Error types |
| `src/hints.rs` | Hint handling |

**KEEP (Generated):**
| File | Purpose |
|------|---------|
| `src/generated/sqlbaselexer.rs` | ANTLR lexer |
| `src/generated/sqlbaseparser.rs` | ANTLR parser |
| `src/generated/sqlbaseparserlistener.rs` | Listener interface |
| `src/generated/mod.rs` | Module exports |

**KEEP (Foundation):**
| File | Purpose |
|------|---------|
| `src/antlr_predicates.rs` | Predicate implementations |
| `src/antlr4rust_workarounds.rs` | Trait re-exports |
| `build_antlr.py` | ANTLR build pipeline |
| `KNOWN_PREDICATES.json` | Predicate catalog |

**CREATE:**
| File | Purpose |
|------|---------|
| `src/antlr_parser.rs` | Parser wrapper |
| `src/ast_converter.rs` | CST to IR converter |

**DELETE:**
| File | Reason |
|------|--------|
| `src/parser.rs` | Replaced by ANTLR |
| `src/keywords.rs` | Keywords from ANTLR |
| `src/functions.rs` | Not needed |
| `src/build_generated.rs` | Not needed |

**MODIFY:**
| File | Changes |
|------|---------|
| `src/formatter.rs` | Use ANTLR parser |
| `src/lib.rs` | Update module exports |
| `build.rs` | Remove keyword extraction |

---

## Verification Checklist

**BEFORE marking complete, ALL must be true:**

- [ ] `cargo build` succeeds with no warnings
- [ ] `cargo test` passes ALL tests (including previously failing)
- [ ] `src/parser.rs` is DELETED
- [ ] `src/keywords.rs` is DELETED
- [ ] `src/functions.rs` is DELETED
- [ ] `src/build_generated.rs` is DELETED
- [ ] NO hand-coded keyword lists exist anywhere
- [ ] NO hand-coded operator lists exist anywhere
- [ ] Scientific notation formats correctly: `SELECT 1.5e10`
- [ ] Nested field access works: `SELECT a.b.c.d.e`
- [ ] CLUSTER BY formats correctly
- [ ] DISTRIBUTE BY formats correctly
- [ ] SORT BY formats correctly
- [ ] Lambda expressions work: `x -> x + 1`
- [ ] Double-colon cast works: `x::INT`
- [ ] All set operations work: UNION, EXCEPT, INTERSECT, MINUS
- [ ] All join types work: LEFT SEMI, LEFT ANTI, NATURAL
- [ ] Comments are preserved in correct positions
- [ ] WASM build succeeds: `cargo build --target wasm32-unknown-unknown -p sparkfmt-wasm`

---

## NON-NEGOTIABLE CONSTRAINTS

1. **Grammar is truth** - If grammar supports it, we support it
2. **No silent drops** - Unknown constructs MUST error, not disappear
3. **No hand-coded lists** - Everything from ANTLR generation
4. **Loud failures** - Unhandled cases fail build/test, never silently corrupt
5. **Complete coverage** - Partial support is a bug
6. **Delete, don't comment** - Remove old code, don't leave commented backups

---

## FORBIDDEN Actions

- ❌ Creating new hand-coded keyword lists
- ❌ Adding `_ => {}` catch-all patterns that drop tokens
- ❌ Commenting out code instead of deleting
- ❌ Keeping "backup" copies of deleted files
- ❌ Modifying grammar files
- ❌ Skipping tests that are "hard to fix"
- ❌ Using `unwrap()` or `expect()` in parser code
- ❌ Ignoring the 3 failing Spark clause tests
