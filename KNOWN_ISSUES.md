# Known Issues - Spark SQL Formatter

This document tracks known formatting issues that still need to be fixed.

**Last Updated:** Session after fixing HIGH/MEDIUM priority issues - 102/102 tests passing

---

## ✅ FIXED Issues

### 1. Complex CASE Statements - Should Multiline ✅ FIXED

**Priority**: HIGH  
**Status**: FIXED - Multiple WHEN branches now properly formatted with indentation

```sql
-- INPUT
select case when status = 1 then 'active' when status = 2 then 'pending' else 'unknown' end from t

-- OUTPUT (NOW CORRECT)
SELECT
     CASE
        WHEN status = 1 THEN 'active'
        WHEN status = 2 THEN 'pending'
        ELSE 'unknown'
     END
FROM t
```

---

### 2. GROUPING SETS / ROLLUP / CUBE ✅ FIXED

**Priority**: MEDIUM  
**Status**: FIXED - Arguments now stay inline

```sql
-- INPUT
select a, sum(x) from t group by grouping sets ((a), (b), ())

-- OUTPUT (NOW CORRECT)
SELECT
     a
    ,SUM(x)
FROM t
GROUP BY GROUPING SETS ((a), (b), ())

-- ROLLUP
GROUP BY ROLLUP(a, b)

-- CUBE
GROUP BY CUBE(a, b)
```

---

### 3. Unary Operators ✅ FIXED

**Priority**: HIGH  
**Status**: FIXED - Unary operators now have no space after them

```sql
-- INPUT
select -x, +y from t

-- OUTPUT (NOW CORRECT)
SELECT
     -x
    ,+y
FROM t
```

---

### 4. Array Access ✅ FIXED

**Priority**: HIGH  
**Status**: FIXED - No spaces around brackets

```sql
-- INPUT
select arr[0], map['key'] from t

-- OUTPUT (NOW CORRECT)
SELECT
     arr[0]
    ,map['key']
FROM t
```

---

### 5. Lambda Expression Spacing ✅ FIXED

**Priority**: MEDIUM  
**Status**: FIXED - Spacing correctly preserved

```sql
-- INPUT
select transform(arr, x -> x + 1) from t

-- OUTPUT (NOW CORRECT)
SELECT TRANSFORM(arr, x -> x + 1)
FROM t
```

---

## Remaining Issues

### 6. Timestamp/Interval Literals

**Priority**: LOW

```sql
-- INPUT
select timestamp '2024-01-01 12:00:00' from t

-- CURRENT OUTPUT (WRONG)
SELECT timestamp 2024 - 01 - 01 12 : 00 : 00 FROM t

-- EXPECTED OUTPUT
SELECT TIMESTAMP '2024-01-01 12:00:00'
FROM t
```

Note: Without the string literal around the timestamp, the parser treats it as arithmetic.

---

### 7. SET Configuration - Uppercasing Config Names

**Priority**: LOW

```sql
-- INPUT
set spark.sql.shuffle.partitions = 200

-- CURRENT OUTPUT (WRONG)
SET spark.SQL.shuffle.PARTITIONS = 200

-- EXPECTED OUTPUT
SET spark.sql.shuffle.partitions = 200
```

---

### 8. MERGE Statement - No Clause Formatting

**Priority**: LOW

```sql
-- INPUT
MERGE INTO target t USING source s ON t.id = s.id WHEN MATCHED THEN UPDATE SET val = s.val

-- CURRENT OUTPUT (WRONG)
MERGE INTO target t USING source s ON t.id = s.id WHEN MATCHED THEN UPDATE SET val = s.val

-- EXPECTED OUTPUT
MERGE INTO target t
USING source s
ON t.id = s.id
WHEN MATCHED THEN UPDATE SET val = s.val
```

---

### 9. Complex Inline Comments

**Priority**: LOW

```sql
-- INPUT
select
    x, -- first column
    y  -- second column
from t

-- CURRENT OUTPUT (comments may shift)
-- Needs investigation

-- EXPECTED OUTPUT
SELECT
     x  -- first column
    ,y  -- second column
FROM t
```

---

## Priority Summary

| Priority | Fixed | Remaining |
|----------|-------|-----------|
| HIGH | 3/3 ✅ | 0 |
| MEDIUM | 2/2 ✅ | 0 |
| LOW | 0 | 4 |

**Overall Progress**: 5/9 issues fixed (all HIGH/MEDIUM priority issues resolved)
