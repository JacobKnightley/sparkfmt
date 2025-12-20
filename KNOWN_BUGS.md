# Known Bugs

Running list of confirmed bugs in sparkfmt. Includes comment handling issues discovered during testing.

---

## CRITICAL: Silent Data Loss Bugs

These bugs silently drop or corrupt user data.

---

## Bug 1: Quoted Identifiers Dropped in Type Parameters

**Status:** Open  
**Severity:** High (silent data loss)  
**Location:** `crates/sparkfmt-core/src/parser.rs` - `collect_balanced()` function

### Description

Quoted identifiers (backtick-wrapped names) inside type parameters are silently dropped due to `_ => {}` catch-all in match statements.

### Reproduction

```sql
-- Input
select cast(x as struct<`field name`: string>) from t

-- Actual Output (BROKEN)
SELECT
     CAST(x AS STRUCT<IELDNAME:STRING>)
FROM t

-- Expected Output
SELECT
     CAST(x AS STRUCT<`field name`:STRING>)
FROM t
```

The quoted identifier `` `field name` `` is completely lost.

### Root Cause

In `collect_balanced()` (line ~1140-1170):

```rust
match token {
    ParserToken::Symbol(s) if s == open => { ... }
    ParserToken::Symbol(s) if s == close => { ... }
    ParserToken::Word(w) => { result.push_str(&w.to_uppercase()); }
    ParserToken::Number(n) => { result.push_str(&n); }
    ParserToken::Symbol(s) => { result.push_str(&s); }
    ParserToken::Eof => { return Err(...); }
    _ => {}  // <-- SILENTLY DROPS QuotedIdentifier, StringLiteral
}
```

### Fix

Add missing cases:

```rust
ParserToken::QuotedIdentifier(s) => {
    result.push('`');
    result.push_str(&s);
    result.push('`');
}
ParserToken::StringLiteral(s) => {
    result.push_str(&s);  // Already includes quotes
}
```

---

## Bug 2: Quoted Identifiers Dropped in SET Assignment Values

**Status:** Open  
**Severity:** Medium  
**Location:** `crates/sparkfmt-core/src/parser.rs` - `parse_set_assignment_value()` function (line ~2215)

### Description

Quoted identifiers in SET statement values are silently dropped.

### Root Cause

Same pattern - `_ => {}` in match statement:

```rust
match lexer.next()? {
    ParserToken::Word(w) => parts.push(w),
    ParserToken::Symbol(s) => parts.push(s),
    ParserToken::Number(n) => parts.push(n),
    ParserToken::StringLiteral(s) => parts.push(s),
    _ => {}  // <-- DROPS QuotedIdentifier
}
```

### Fix

Add `ParserToken::QuotedIdentifier` case.

---

## Bug 3: Quoted Identifiers Dropped in MERGE Clauses

**Status:** Open  
**Severity:** Medium  
**Location:** `crates/sparkfmt-core/src/parser.rs` - MERGE clause parsing (line ~2515)

### Description

Quoted identifiers in MERGE WHEN MATCHED/NOT MATCHED clauses are silently dropped.

### Root Cause

Same pattern - `_ => {}` in match statement.

### Fix

Add `ParserToken::QuotedIdentifier` case.

---

## Bug 4: Quoted Identifiers Dropped in Simple Type Parsing

**Status:** Open  
**Severity:** Low (edge case)  
**Location:** `crates/sparkfmt-core/src/parser.rs` - `parse_type_string()` function (line ~1135)

### Description

Same issue but in the simpler type parsing path.

### Root Cause

```rust
_ => {}
```

### Fix

Add missing token cases.

---

## Pattern Summary

All four bugs share the same anti-pattern:

```rust
match token {
    // ... some cases handled ...
    _ => {}  // Silent drop - DANGEROUS
}
```

**Correct pattern** should be one of:

1. **Handle all cases explicitly:**
   ```rust
   ParserToken::QuotedIdentifier(s) => { ... }
   ParserToken::StringLiteral(s) => { ... }
   ```

2. **Or fail on unknown:**
   ```rust
   _ => return Err(FormatError::new(format!("Unexpected token: {:?}", token)))
   ```

3. **Or at minimum, preserve the token as-is:**
   ```rust
   _ => { /* log warning, but don't lose data */ }
   ```

---

## Verification

After fixes, these should all work:

```bash
# Quoted identifier in type
echo "select cast(x as struct<\`field name\`: string>) from t" | cargo run --release --bin sparkfmt -- -
# Expected: STRUCT<`field name`:STRING>

# Quoted identifier in nested type
echo "select cast(x as map<\`key col\`, array<\`val col\`>>) from t" | cargo run --release --bin sparkfmt -- -

# Multiple quoted identifiers
echo "select cast(x as struct<\`a b\`: int, \`c d\`: string>) from t" | cargo run --release --bin sparkfmt -- -
```

---

## Bug 5: Scientific Notation Exponent Dropped

**Status:** Open  
**Severity:** Critical (silent data loss)  
**Location:** Lexer number parsing

### Reproduction

```sql
-- Input
select x from t where x = 1.5e10

-- Actual Output (BROKEN)
WHERE x=1.5

-- Expected Output
WHERE x=1.5e10
```

The exponent `e10` is silently dropped, changing the value from 15,000,000,000 to 1.5.

---

## Bug 6: Nested Field Access Truncated (More Than 2 Levels)

**Status:** Open  
**Severity:** Critical (silent data loss)  
**Location:** `crates/sparkfmt-core/src/parser.rs` - field access parsing

### Reproduction

```sql
-- Input
select x.y.z from t

-- Actual Output (BROKEN)
SELECT
     x.y

-- Expected Output
SELECT
     x.y.z
```

Only first two levels preserved. Third and beyond dropped silently.

---

## Bug 7: Hex Literals Parsed Incorrectly

**Status:** Open  
**Severity:** Critical (semantic change)  
**Location:** Lexer

### Reproduction

```sql
-- Input
select 0x1F from t

-- Actual Output (BROKEN)
SELECT
     0 AS x1F

-- Expected Output
SELECT
     0x1F
```

`0x1F` (hex 31) becomes `0 AS x1F` - completely different meaning!

---

## Bug 8: Binary Literals Parsed Incorrectly

**Status:** Open  
**Severity:** Critical (semantic change)  
**Location:** Lexer

### Reproduction

```sql
-- Input
select 0b1010 from t

-- Actual Output (BROKEN)
SELECT
     0 AS b1010

-- Expected Output
SELECT
     0b1010
```

---

## Bug 9: Double-Quoted Identifiers Not Supported

**Status:** Open  
**Severity:** High (fails to parse)  
**Location:** Lexer

### Reproduction

```sql
-- Input
select "column name" from t

-- Actual Output
Error: Unexpected character at position 7
```

Standard SQL double-quoted identifiers fail completely.

---

## MEDIUM: Unsupported SQL Constructs (Partial Output)

These output partial/truncated SQL but don't completely fail.

---

## Bug 10: EXCEPT Not Supported

**Status:** Open  
**Severity:** Medium  

### Reproduction

```sql
-- Input
select * from t except select * from s

-- Actual Output (BROKEN)
SELECT
     *
FROM t except
```

---

## Bug 11: INTERSECT Not Supported

**Status:** Open  
**Severity:** Medium  

### Reproduction

```sql
-- Input
select * from t intersect select * from s

-- Actual Output (BROKEN)
SELECT
     *
FROM t intersect
```

---

## Bug 12: QUALIFY Clause Not Supported

**Status:** Open  
**Severity:** Medium (Spark SQL feature)

### Reproduction

```sql
-- Input
select x from t qualify row_number() over (partition by y order by z) = 1

-- Actual Output (BROKEN)
SELECT
     x
FROM t qualify
```

---

## Bug 13: CLUSTER BY Not Supported

**Status:** Open  
**Severity:** Medium (Spark SQL feature)

### Reproduction

```sql
-- Input
select x from t cluster by x

-- Actual Output (BROKEN)
SELECT
     x
FROM t cluster
```

---

## Bug 14: DISTRIBUTE BY Not Supported

**Status:** Open  
**Severity:** Medium (Spark SQL feature)

### Reproduction

```sql
-- Input  
select x from t distribute by x

-- Actual Output (BROKEN)
SELECT
     x
FROM t distribute
```

---

## Bug 15: SORT BY Not Supported

**Status:** Open  
**Severity:** Medium (Spark SQL feature)

### Reproduction

```sql
-- Input
select x from t sort by x

-- Actual Output (BROKEN)
SELECT
     x
FROM t sort
```

---

## Bug 16: Lambda Expressions Not Supported

**Status:** Open  
**Severity:** Medium (returns original)

### Reproduction

```sql
-- Input
select transform(arr, x -> x + 1) from t

-- Actual Output
select transform(arr, x -> x + 1) from t
```

Falls back to original (safe but not formatted).

---

---

## Comment Handling Bugs

These bugs relate to comment preservation during formatting.

---

## Bug 17: Comment After LIMIT Clause Dropped

**Status:** Open  
**Severity:** High (silent data loss)  
**Location:** `crates/sparkfmt-core/src/parser.rs` - LIMIT parsing

### Description

Comments appearing after the LIMIT clause are silently dropped (not even emitted as fallback).

### Reproduction

```sql
-- Input
select x from t limit 10 -- comment on limit

-- Actual Output (BROKEN)
SELECT
     x
FROM t
LIMIT 10

-- Expected Output
SELECT
     x
FROM t
LIMIT 10 -- comment on limit
```

The comment `-- comment on limit` is completely lost - not even preserved as a fallback comment at the end of the query.

---

## Bug 18: Comments After GROUP BY/ORDER BY Items Not Attached Inline

**Status:** Open  
**Severity:** Low (placement, not loss)  
**Location:** `crates/sparkfmt-core/src/formatter.rs`

### Description

Comments on the same line as GROUP BY or ORDER BY items are preserved but emitted as fallback comments at the end, rather than being attached inline to the clause item.

### Reproduction

```sql
-- Input
select x from t group by x -- comment on group by

-- Actual Output (suboptimal but not lost)
SELECT
     x
FROM t
GROUP BY
     x
-- comment on group by

-- Expected Output
SELECT
     x
FROM t
GROUP BY
     x -- comment on group by
```

Comment is preserved but placement is not ideal.

---

## Bug 19: Comments After FROM Table Not Attached Inline

**Status:** Open  
**Severity:** Low (placement, not loss)  
**Location:** `crates/sparkfmt-core/src/formatter.rs`

### Description

Comments on the same line as the FROM table are preserved but emitted as fallback comments at the end.

### Reproduction

```sql
-- Input
select x from t -- comment on table

-- Actual Output (suboptimal but not lost)
SELECT
     x
FROM t
-- comment on table

-- Expected Output
SELECT
     x
FROM t -- comment on table
```

---

## Bug 20: Block Comments Before SELECT Moved to End

**Status:** Open  
**Severity:** Low (placement, not loss)  
**Location:** `crates/sparkfmt-core/src/formatter.rs`

### Description

Block comments that appear before SELECT are moved to the end of the query as fallback comments, rather than being emitted as leading comments.

### Reproduction

```sql
-- Input
/* header comment */ select x from t

-- Actual Output (suboptimal)
SELECT
     x
FROM t
/* header comment */

-- Expected Output
/* header comment */
SELECT
     x
FROM t
```

Note: Line comments (`--`) before SELECT ARE correctly preserved as leading comments.

---

## Summary by Severity

| Severity | Count | Description |
|----------|-------|-------------|
| Critical | 5 | Silent data loss or semantic corruption |
| High | 2 | Complete data loss or parse failure |
| Medium | 7 | Unsupported constructs (partial output) |
| Low | 4 | Comment placement issues (preserved but not ideal) |

### Critical Bugs (Fix First)

1. **Bug 5** - Scientific notation drops exponent
2. **Bug 6** - Nested field access truncated  
3. **Bug 7** - Hex literals corrupted
4. **Bug 8** - Binary literals corrupted
5. **Bugs 1-4** - Quoted identifiers dropped in various contexts

### High Priority

6. **Bug 9** - Double-quoted identifiers fail to parse
7. **Bug 17** - Comment after LIMIT dropped

### Pattern: Silent `_ => {}`

Bugs 1-4 share the same root cause: match statements with `_ => {}` that silently discard unhandled tokens.

### Pattern: Lexer Number Handling

Bugs 5, 7, 8 are all lexer issues with special number formats.

### Pattern: Comment Attachment

Bugs 17-20 relate to comment attachment - some clauses don't properly capture trailing comments, causing them to either be lost (Bug 17) or moved to fallback positions (Bugs 18-20).
