# Known Issues - Spark SQL Formatter

This document tracks known formatting issues that still need to be fixed.

**Last Updated:** All HIGH/MEDIUM priority issues fixed - 106/106 tests passing

---

## Remaining Issues

### 1. Timestamp/Interval Literals

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

### 2. SET Configuration - Uppercasing Config Names

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

### 3. MERGE Statement - No Clause Formatting

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

### 4. Complex Inline Comments

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

| Priority | Remaining |
|----------|-----------|
| HIGH | 0 ✅ |
| MEDIUM | 0 ✅ |
| LOW | 4 |

**Overall Progress**: All HIGH/MEDIUM priority issues resolved. 4 LOW priority issues remain.
