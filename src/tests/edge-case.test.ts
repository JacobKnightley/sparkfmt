/**
 * Edge Case Tests - Known Bugs and Edge Cases
 * 
 * This file documents known formatting bugs discovered during edge case testing.
 * Each test case includes a detailed comment explaining:
 * - The input SQL
 * - The expected correct output
 * - The actual buggy output
 * 
 * These tests are expected to FAIL until the bugs are fixed.
 * Run with: npm test -- -v to see failure details
 * 
 * Note: Only valid Spark SQL syntax is tested. Features from other SQL dialects
 * (PostgreSQL, MySQL, T-SQL, Snowflake, etc.) are excluded.
 */
import { TestSuite } from './framework.js';

export const edgeCaseBugs: TestSuite = {
    name: 'Edge Case Bugs (Known Issues)',
    tests: [
        // =====================================================================
        // BUG #1: Bitwise NOT operator adds spurious space
        // =====================================================================
        // Input:  select ~a from t
        // Expect: SELECT ~a FROM t
        // Actual: SELECT ~ a FROM t
        // Issue:  Space incorrectly inserted after ~ operator
        {
            name: 'Bitwise NOT should not have space after tilde',
            input: 'select ~a from t',
            expected: 'SELECT ~a FROM t',
        },

        // =====================================================================
        // BUG #2: Decimal ending with dot loses space before FROM
        // =====================================================================
        // Input:  select 1. from dual
        // Expect: SELECT 1. FROM dual
        // Actual: SELECT 1.FROM dual
        // Issue:  Space missing between "1." and "FROM"
        {
            name: 'Decimal ending with dot should have space before FROM',
            input: 'select 1. from dual',
            expected: 'SELECT 1. FROM dual',
        },

        // =====================================================================
        // BUG #3: Double-colon cast to ARRAY type adds spurious spaces
        // =====================================================================
        // Input:  select x::array<int> from t
        // Expect: SELECT x::ARRAY<INT> FROM t
        // Actual: SELECT x::ARRAY < INT > FROM t
        // Issue:  Spaces added around angle brackets in type syntax
        {
            name: 'Double-colon cast to array type should not have spaces around brackets',
            input: 'select x::array<int> from t',
            expected: 'SELECT x::ARRAY<INT> FROM t',
        },

        // =====================================================================
        // BUG #4: Double-colon cast to MAP type is completely broken
        // =====================================================================
        // Input:  select x::map<string, int> from t
        // Expect: SELECT x::MAP<STRING, INT> FROM t
        // Actual: SELECT x::MAP < STRING
        //             ,INT >
        //         FROM t
        // Issue:  Commas in type params treated as list separators
        {
            name: 'Double-colon cast to map type should preserve type syntax',
            input: 'select x::map<string, int> from t',
            expected: 'SELECT x::MAP<STRING, INT> FROM t',
        },

        // =====================================================================
        // BUG #5: Double-colon cast to STRUCT type is broken
        // =====================================================================
        // Input:  select x::struct<a:int, b:string> from t
        // Expect: SELECT x::STRUCT<a:INT, b:STRING> FROM t
        // Actual: SELECT x::STRUCT < a :INT
        //             ,b :STRING >
        //         FROM t
        // Issue:  Commas in struct fields treated as list separators
        {
            name: 'Double-colon cast to struct type should preserve type syntax',
            input: 'select x::struct<a:int, b:string> from t',
            expected: 'SELECT x::STRUCT<a:INT, b:STRING> FROM t',
        },

        // =====================================================================
        // BUG #6: Named window with modification inserts spurious AS
        // =====================================================================
        // Input:  select sum(x) over (w rows 1 preceding) from t window w as (order by z)
        // Expect: SELECT SUM(x) OVER (w ROWS 1 PRECEDING) FROM t WINDOW w AS (ORDER BY z)
        // Actual: SELECT SUM(x) AS over (w ROWS 1 PRECEDING) FROM t WINDOW w AS (ORDER BY z)
        // Issue:  AS inserted before OVER when using named window with modification
        {
            name: 'Named window with modification should not insert AS before OVER',
            input: 'select sum(x) over (w rows 1 preceding) from t window w as (partition by y order by z)',
            expected: 'SELECT SUM(x) OVER (w ROWS 1 PRECEDING)\nFROM t\nWINDOW w AS (PARTITION BY y ORDER BY z)',
        },

        // =====================================================================
        // BUG #7: TABLESAMPLE BUCKET partially lowercase
        // =====================================================================
        // Input:  select * from t tablesample bucket 1 out of 10
        // Expect: SELECT * FROM t TABLESAMPLE BUCKET 1 OUT OF 10
        // Actual: SELECT * FROM t tablesample BUCKET 1 OUT OF 10
        // Issue:  TABLESAMPLE not uppercased before BUCKET syntax
        {
            name: 'TABLESAMPLE BUCKET should be fully uppercased',
            input: 'select * from t tablesample bucket 1 out of 10',
            expected: 'SELECT * FROM t TABLESAMPLE BUCKET 1 OUT OF 10',
        },

        // =====================================================================
        // BUG #8: Simple CASE expression has incorrect newline placement
        // =====================================================================
        // Input:  select case x when 1 then a when 2 then b else c end from t
        // Expect: SELECT CASE x
        //             WHEN 1 THEN a
        //             WHEN 2 THEN b
        //             ELSE c
        //          END FROM t
        // Actual: SELECT
        //              CASE
        //         x
        //              WHEN 1 THEN a
        //              ...
        // Issue:  The value after CASE (x) goes to its own line without indentation
        {
            name: 'Simple CASE expression value should be on same line as CASE',
            input: 'select case x when 1 then a when 2 then b else c end from t',
            expected: 'SELECT\n     CASE x\n        WHEN 1 THEN a\n        WHEN 2 THEN b\n        ELSE c\n     END\nFROM t',
        },

        // =====================================================================
        // BUG #9: Implicit cross join has spaces around commas
        // =====================================================================
        // Input:  select * from a, b, c
        // Expect: SELECT * FROM a, b, c
        // Actual: SELECT * FROM a , b , c
        // Issue:  Extra space before comma in table list
        {
            name: 'Implicit cross join should not have space before comma',
            input: 'select * from a, b, c',
            expected: 'SELECT * FROM a, b, c',
        },

        // =====================================================================
        // BUG #10: ALL/ANY/SOME keywords not uppercased in comparisons
        // =====================================================================
        // Input:  select * from t where x > all (select y from t2)
        // Expect: SELECT * FROM t WHERE x > ALL (SELECT y FROM t2)
        // Actual: SELECT * FROM t WHERE x > all (SELECT y FROM t2)
        // Issue:  ALL not uppercased
        {
            name: 'ALL keyword in comparison should be uppercased',
            input: 'select * from t where x > all (select y from t2)',
            expected: 'SELECT * FROM t WHERE x > ALL (SELECT y FROM t2)',
        },
        {
            name: 'ANY keyword in comparison should be uppercased',
            input: 'select * from t where x > any (select y from t2)',
            expected: 'SELECT * FROM t WHERE x > ANY (SELECT y FROM t2)',
        },
        {
            name: 'SOME keyword in comparison should be uppercased',
            input: 'select * from t where x > some (select y from t2)',
            expected: 'SELECT * FROM t WHERE x > SOME (SELECT y FROM t2)',
        },

        // =====================================================================
        // BUG #11: Empty parens as column has lowercase FROM
        // =====================================================================
        // Input:  select () from t
        // Expect: SELECT () FROM t
        // Actual: SELECT () from t
        // Issue:  FROM not uppercased after empty parentheses
        {
            name: 'Empty parens should not prevent FROM uppercasing',
            input: 'select () from t',
            expected: 'SELECT () FROM t',
        },

        // =====================================================================
        // BUG #12: ANALYZE TABLE FOR COLUMNS has wrong comma spacing
        // =====================================================================
        // Input:  analyze table t compute statistics for columns a, b
        // Expect: ANALYZE TABLE t COMPUTE STATISTICS FOR COLUMNS a, b
        // Actual: ANALYZE TABLE t COMPUTE STATISTICS FOR COLUMNS a , b
        // Issue:  Space before comma in column list
        {
            name: 'ANALYZE TABLE column list should not have space before comma',
            input: 'analyze table t compute statistics for columns a, b',
            expected: 'ANALYZE TABLE t COMPUTE STATISTICS FOR COLUMNS a, b',
        },

        // =====================================================================
        // BUG #13: ANALYZE TABLE NOSCAN not uppercased
        // =====================================================================
        // Input:  analyze table t compute statistics noscan
        // Expect: ANALYZE TABLE t COMPUTE STATISTICS NOSCAN
        // Actual: ANALYZE TABLE t COMPUTE STATISTICS noscan
        // Issue:  NOSCAN not uppercased
        {
            name: 'NOSCAN should be uppercased',
            input: 'analyze table t compute statistics noscan',
            expected: 'ANALYZE TABLE t COMPUTE STATISTICS NOSCAN',
        },

        // =====================================================================
        // BUG #14: LATERAL VIEW AS not uppercased
        // =====================================================================
        // Input:  select a, b from t lateral view explode(arr) as x
        // Expect: SELECT ... FROM t LATERAL VIEW EXPLODE(arr) AS x
        // Actual: SELECT ... FROM t LATERAL VIEW EXPLODE(arr) as x
        // Issue:  AS not uppercased in LATERAL VIEW
        {
            name: 'LATERAL VIEW AS should be uppercased',
            input: 'select * from t lateral view explode(arr) as x',
            expected: 'SELECT * FROM t LATERAL VIEW EXPLODE(arr) AS x',
        },

        // =====================================================================
        // BUG #15: Complex types in DDL completely broken
        // =====================================================================
        // Input:  create table t (a array<int>, b map<string, int>, c struct<x:int, y:string>)
        // Expect: CREATE TABLE t (
        //              a ARRAY<INT>
        //             ,b MAP<STRING, INT>
        //             ,c STRUCT<x:INT, y:STRING>
        //         )
        // Actual: CREATE TABLE t (
        //              a ARRAY < INT >
        //             ,b MAP < STRING
        //             ,INT >
        //             ,c STRUCT < x :INT
        //             ,y :STRING >
        //         )
        // Issue:  Complex type syntax completely mangled - commas in types parsed as column separators
        {
            name: 'Complex types in DDL should preserve type syntax',
            input: 'create table t (a array<int>, b map<string, int>, c struct<x:int, y:string>)',
            expected: 'CREATE TABLE t (\n     a ARRAY<INT>\n    ,b MAP<STRING, INT>\n    ,c STRUCT<x:INT, y:STRING>\n)',
        },

        // =====================================================================
        // BUG #16: Nested complex types in DDL produce garbage
        // =====================================================================
        // Input:  create table t (a array<array<int>>, b map<string, struct<x:int>>)
        // Expect: CREATE TABLE t (
        //              a ARRAY<ARRAY<INT>>
        //             ,b MAP<STRING, STRUCT<x:INT>>
        //         )
        // Actual: CREATE TABLE t (
        //              a ARRAY < ARRAY < INT >>,
        //                 < STRING,
        //               < X :INT >>) <EOF>
        // Issue:  Nested complex types produce completely invalid SQL
        {
            name: 'Nested complex types in DDL should not produce garbage',
            input: 'create table t (a array<array<int>>, b map<string, struct<x:int>>)',
            expected: 'CREATE TABLE t (\n     a ARRAY<ARRAY<INT>>\n    ,b MAP<STRING, STRUCT<x:INT>>\n)',
        },

        // =====================================================================
        // BUG #17: EXTRACT with non-standard field not uppercased
        // =====================================================================
        // Input:  select extract(dayofweek from x) from t
        // Expect: SELECT EXTRACT(DAYOFWEEK FROM x) FROM t
        // Actual: SELECT EXTRACT(dayofweek FROM x) FROM t
        // Issue:  DAYOFWEEK not uppercased inside EXTRACT
        {
            name: 'EXTRACT field DAYOFWEEK should be uppercased',
            input: 'select extract(dayofweek from x) from t',
            expected: 'SELECT EXTRACT(DAYOFWEEK FROM x) FROM t',
        },

        // =====================================================================
        // BUG #18: Subquery in EXISTS/IN has spurious newline before close paren
        // =====================================================================
        // Input:  select * from t where exists (select 1)
        // Expect: SELECT * FROM t WHERE EXISTS (SELECT 1)
        // Actual: SELECT * FROM t WHERE EXISTS (SELECT 1
        //         )
        // Issue:  Newline inserted before closing paren of subquery
        {
            name: 'EXISTS subquery should not have newline before close paren',
            input: 'select * from t where exists (select 1)',
            expected: 'SELECT * FROM t WHERE EXISTS (SELECT 1)',
        },

        // =====================================================================
        // BUG #19: Scalar subquery in SELECT has spurious newline
        // =====================================================================
        // Input:  select (select max(x) from t2) from t
        // Expect: SELECT (SELECT MAX(x) FROM t2) FROM t
        // Actual: SELECT (SELECT MAX(x) FROM t2
        //         ) FROM t
        // Issue:  Newline inserted before closing paren of scalar subquery
        {
            name: 'Scalar subquery should not have newline before close paren',
            input: 'select (select max(x) from t2) from t',
            expected: 'SELECT (SELECT MAX(x) FROM t2) FROM t',
        },

        // =====================================================================
        // BUG #20: Subquery in FROM has spurious newline
        // =====================================================================
        // Input:  select * from (select * from t) sub
        // Expect: SELECT * FROM (SELECT * FROM t) sub
        // Actual: SELECT * FROM (SELECT * FROM t
        //         ) sub
        // Issue:  Newline inserted before closing paren
        {
            name: 'Subquery in FROM should not have newline before close paren',
            input: 'select * from (select * from t) sub',
            expected: 'SELECT * FROM (SELECT * FROM t) sub',
        },

        // =====================================================================
        // BUG #21: CTE has spurious newline before close paren
        // =====================================================================
        // Input:  with a as (select 1) select * from a
        // Expect: WITH a AS (SELECT 1) SELECT * FROM a
        // Actual: WITH a AS (SELECT 1
        //         ) SELECT * FROM a
        // Issue:  Newline inserted before closing paren in CTE
        {
            name: 'CTE should not have newline before close paren',
            input: 'with a as (select 1) select * from a',
            expected: 'WITH a AS (SELECT 1) SELECT * FROM a',
        },

        // =====================================================================
        // BUG #22: UNION with parens has spurious newlines
        // =====================================================================
        // Input:  (select 1) union (select 2)
        // Expect: (SELECT 1) UNION (SELECT 2)
        // Actual: (SELECT 1
        //         )
        //         UNION
        //         (SELECT 2
        //         )
        // Issue:  Newlines inserted in set operation subqueries
        {
            name: 'UNION with parens should not have spurious newlines',
            input: '(select 1) union (select 2)',
            expected: '(SELECT 1) UNION (SELECT 2)',
        },

        // =====================================================================
        // BUG #23: AT TIME ZONE incorrectly becomes AS at TIME ZONE
        // =====================================================================
        // Input:  select x at time zone 'UTC' from t
        // Expect: SELECT x AT TIME ZONE 'UTC' FROM t
        // Actual: SELECT x AS at TIME ZONE 'UTC' FROM t
        // Issue:  AT incorrectly treated as alias causing AS insertion
        {
            name: 'AT TIME ZONE should not insert AS keyword',
            input: "select x at time zone 'UTC' from t",
            expected: "SELECT x AT TIME ZONE 'UTC' FROM t",
        },

        // =====================================================================
        // BUG #24: EXCEPT columns on qualified star breaks formatting
        // =====================================================================
        // Input:  select t.* except (a, b) from t
        // Expect: SELECT t.* EXCEPT (a, b) FROM t
        // Actual: SELECT t.* EXCEPT (a
        //             ,b)
        //         FROM t
        // Issue:  Column list in EXCEPT incorrectly expanded to multiple lines
        {
            name: 'EXCEPT columns on qualified star should not expand',
            input: 'select t.* except (a, b) from t',
            expected: 'SELECT t.* EXCEPT (a, b) FROM t',
        },

        // =====================================================================
        // BUG #25: VALUES without SELECT has odd formatting
        // =====================================================================
        // Input:  values 1, 2, 3
        // Expect: VALUES 1, 2, 3
        // Actual: VALUES
        //         1
        //         ,2
        //         ,3
        // Issue:  Values should stay on one line for simple values
        {
            name: 'VALUES without SELECT should stay inline for simple values',
            input: 'values 1, 2, 3',
            expected: 'VALUES 1, 2, 3',
        },

        // =====================================================================
        // BUG #26: Dollar-quoted strings should be preserved
        // =====================================================================
        // Input:  select $$hello world$$ from t
        // Note:   Dollar-quoting is valid in Spark SQL for string literals
        // Expect: SELECT $$hello world$$ FROM t
        // Actual: Lexer error or malformed output
        {
            name: 'Dollar-quoted strings should be preserved',
            input: 'select $$hello world$$ from t',
            expected: 'SELECT $$hello world$$ FROM t',
        },

        // =====================================================================
        // BUG #27: IGNORE NULLS adds space before function name parens
        // =====================================================================
        // Input:  select first_value(x ignore nulls) over (order by y) from t
        // Expect: SELECT FIRST_VALUE(x IGNORE NULLS) OVER (ORDER BY y) FROM t
        // Actual: SELECT first_value (x IGNORE NULLS) OVER (ORDER BY y) FROM t
        // Issue:  Space before opening paren and function not uppercased
        {
            name: 'IGNORE NULLS should not add space before function parens',
            input: 'select first_value(x ignore nulls) over (order by y) from t',
            expected: 'SELECT FIRST_VALUE(x IGNORE NULLS) OVER (ORDER BY y) FROM t',
        },

        // =====================================================================
        // BUG #28: RESPECT NULLS adds space before function name parens
        // =====================================================================
        // Input:  select last_value(x respect nulls) over (order by y) from t
        // Expect: SELECT LAST_VALUE(x RESPECT NULLS) OVER (ORDER BY y) FROM t
        // Actual: SELECT last_value (x RESPECT NULLS) OVER (ORDER BY y) FROM t
        // Issue:  Space before opening paren and function not uppercased
        {
            name: 'RESPECT NULLS should not add space before function parens',
            input: 'select last_value(x respect nulls) over (order by y) from t',
            expected: 'SELECT LAST_VALUE(x RESPECT NULLS) OVER (ORDER BY y) FROM t',
        },

        // =====================================================================
        // BUG #29: TRY_CAST has space before parens
        // =====================================================================
        // Input:  select try_cast(x as int) from t
        // Expect: SELECT TRY_CAST(x AS INT) FROM t
        // Actual: SELECT try_cast (x AS INT) FROM t
        // Issue:  Space before parens and function not uppercased
        {
            name: 'TRY_CAST should be uppercased with no space before parens',
            input: 'select try_cast(x as int) from t',
            expected: 'SELECT TRY_CAST(x AS INT) FROM t',
        },

        // =====================================================================
        // BUG #30: TRANSFORM function has space before parens
        // =====================================================================
        // Input:  select transform(a, b) using 'script.py' as (c, d) from t
        // Expect: SELECT TRANSFORM(a, b) USING 'script.py' AS (c, d) FROM t
        // Actual: SELECT TRANSFORM (a, b) USING 'script.py' AS (c, d) FROM t
        // Issue:  Space before opening parenthesis
        {
            name: 'TRANSFORM function should not have space before parens',
            input: "select transform(a, b) using 'script.py' as (c, d) from t",
            expected: "SELECT TRANSFORM(a, b) USING 'script.py' AS (c, d)\nFROM t",
        },

        // =====================================================================
        // BUG #31: OPTIMIZE ZORDER BY not fully uppercased (Delta Lake)
        // =====================================================================
        // Input:  optimize t zorder by (a, b)
        // Expect: OPTIMIZE t ZORDER BY (a, b)
        // Actual: optimize t zorder BY (a, b)
        // Issue:  OPTIMIZE and ZORDER not uppercased
        {
            name: 'OPTIMIZE ZORDER BY should be fully uppercased',
            input: 'optimize t zorder by (a, b)',
            expected: 'OPTIMIZE t ZORDER BY (a, b)',
        },

        // =====================================================================
        // BUG #32: FORMAT in CAST doubles AS keyword
        // =====================================================================
        // Input:  select cast(x as string format 'YYYY-MM-DD') from t
        // Expect: SELECT CAST(x AS STRING FORMAT 'YYYY-MM-DD') FROM t
        // Actual: SELECT CAST(x AS AS string FORMAT 'YYYY-MM-DD') FROM t
        // Issue:  Double AS keyword and string not uppercased
        {
            name: 'FORMAT in CAST should not double AS keyword',
            input: "select cast(x as string format 'YYYY-MM-DD') from t",
            expected: "SELECT CAST(x AS STRING FORMAT 'YYYY-MM-DD') FROM t",
        },

        // =====================================================================
        // BUG #33: LATERAL subquery has space after comma before LATERAL
        // =====================================================================
        // Input:  select * from t, lateral (select * from s where s.id = t.id)
        // Expect: SELECT * FROM t, LATERAL (SELECT * FROM s WHERE s.id = t.id)
        // Actual: SELECT * FROM t , LATERAL (SELECT * FROM s WHERE s.id = t.id
        //         )
        // Issue:  Space before comma and newline before closing paren
        {
            name: 'LATERAL subquery should not have space before comma',
            input: 'select * from t, lateral (select * from s where s.id = t.id)',
            expected: 'SELECT * FROM t, LATERAL (SELECT * FROM s WHERE s.id = t.id)',
        },

        // =====================================================================
        // BUG #34: UNPIVOT AS not uppercased
        // =====================================================================
        // Input:  select * from t unpivot (val for name in (a as 'A', b as 'B'))
        // Expect: SELECT * FROM t UNPIVOT (val FOR name IN (a AS 'A', b AS 'B'))
        // Actual: SELECT * FROM t UNPIVOT (val FOR name IN (a as 'A', b as 'B'))
        // Issue:  AS keywords inside UNPIVOT IN clause not uppercased
        {
            name: 'UNPIVOT AS aliases should be uppercased',
            input: "select * from t unpivot (val for name in (a as 'A', b as 'B'))",
            expected: "SELECT * FROM t UNPIVOT (val FOR name IN (a AS 'A', b AS 'B'))",
        },

        // =====================================================================
        // BUG #35: Double negative -- becomes line comment
        // =====================================================================
        // Input:  select --5 from t
        // Expect: SELECT - -5 FROM t (or SELECT --5 FROM t if valid)
        // Actual: SELECT --5 from t (treated as comment, loses FROM uppercasing)
        // Issue:  --5 interpreted as line comment eating rest of query
        {
            name: 'Double negative should not become line comment',
            input: 'select --5 from t',
            expected: 'SELECT - -5 FROM t',
        },

        // =====================================================================
        // BUG #36: CREATE FUNCTION has space before parens
        // =====================================================================
        // Input:  create function f(x int) returns int return x + 1
        // Expect: CREATE FUNCTION f(x INT) RETURNS INT RETURN x + 1
        // Actual: CREATE FUNCTION f (x INT) RETURNS INT RETURN x + 1
        // Issue:  Space before parameter list parens
        {
            name: 'CREATE FUNCTION should not have space before parens',
            input: 'create function f(x int) returns int return x + 1',
            expected: 'CREATE FUNCTION f(x INT) RETURNS INT RETURN x + 1',
        },

        // =====================================================================
        // BUG #37: TABLESAMPLE PERCENT partially lowercase
        // =====================================================================
        // Input:  select * from t tablesample (10 percent)
        // Expect: SELECT * FROM t TABLESAMPLE (10 PERCENT)
        // Actual: SELECT * FROM t tablesample (10 percent)
        // Issue:  TABLESAMPLE and PERCENT not uppercased
        {
            name: 'TABLESAMPLE PERCENT should be fully uppercased',
            input: 'select * from t tablesample (10 percent)',
            expected: 'SELECT * FROM t TABLESAMPLE (10 PERCENT)',
        },

        // =====================================================================
        // BUG #38: TABLESAMPLE ROWS partially lowercase
        // =====================================================================
        // Input:  select * from t tablesample (5 rows)
        // Expect: SELECT * FROM t TABLESAMPLE (5 ROWS)
        // Actual: SELECT * FROM t tablesample (5 ROWS)
        // Issue:  TABLESAMPLE not uppercased
        {
            name: 'TABLESAMPLE ROWS should be fully uppercased',
            input: 'select * from t tablesample (5 rows)',
            expected: 'SELECT * FROM t TABLESAMPLE (5 ROWS)',
        },

        // =====================================================================
        // BUG #39: Multiple tables in FROM has space before comma
        // =====================================================================
        // Input:  select * from a, b, c where a.id = b.id and b.id = c.id
        // Expect: SELECT *
        //         FROM a, b, c
        //         WHERE
        //             a.id = b.id
        //             AND b.id = c.id
        // Actual: SELECT *
        //         FROM a , b , c
        //         ...
        // Issue:  Space before commas in table list
        {
            name: 'Multiple tables in FROM should not have space before comma',
            input: 'select * from a, b, c where a.id = b.id and b.id = c.id',
            expected: 'SELECT *\nFROM a, b, c\nWHERE\n    a.id = b.id\n    AND b.id = c.id',
        },

        // =====================================================================
        // BUG #40: VACUUM not uppercased (Delta Lake)
        // =====================================================================
        // Input:  vacuum t
        // Expect: VACUUM t
        // Actual: vacuum t
        // Issue:  VACUUM keyword not uppercased
        {
            name: 'VACUUM should be uppercased',
            input: 'vacuum t',
            expected: 'VACUUM t',
        },

        // =====================================================================
        // BUG #41: VACUUM RETAIN partially uppercased (Delta Lake)
        // =====================================================================
        // Input:  vacuum t retain 168 hours
        // Expect: VACUUM t RETAIN 168 HOURS
        // Actual: vacuum t retain 168 HOURS
        // Issue:  VACUUM and RETAIN not uppercased
        {
            name: 'VACUUM RETAIN should be fully uppercased',
            input: 'vacuum t retain 168 hours',
            expected: 'VACUUM t RETAIN 168 HOURS',
        },

        // =====================================================================
        // BUG #42: SHOW SYSTEM FUNCTIONS partially uppercased
        // =====================================================================
        // Input:  show system functions
        // Expect: SHOW SYSTEM FUNCTIONS
        // Actual: SHOW system FUNCTIONS
        // Issue:  SYSTEM keyword not uppercased
        {
            name: 'SHOW SYSTEM FUNCTIONS should be fully uppercased',
            input: 'show system functions',
            expected: 'SHOW SYSTEM FUNCTIONS',
        },

        // =====================================================================
        // BUG #43: RESTORE TABLE not uppercased (Delta Lake)
        // =====================================================================
        // Input:  restore table t to version as of 1
        // Expect: RESTORE TABLE t TO VERSION AS OF 1
        // Actual: restore TABLE t TO VERSION AS OF 1
        // Issue:  RESTORE keyword not uppercased
        {
            name: 'RESTORE TABLE should be uppercased',
            input: 'restore table t to version as of 1',
            expected: 'RESTORE TABLE t TO VERSION AS OF 1',
        },

        // =====================================================================
        // BUG #44: Multiple semicolons collapse to one
        // =====================================================================
        // Input:  ;;;
        // Expect: ;;;
        // Actual: ;
        // Issue:  Empty statements between semicolons are lost
        {
            name: 'Multiple semicolons should not collapse',
            input: ';;;',
            expected: ';;;',
        },

        // =====================================================================
        // BUG #45: Leading semicolon is lost
        // =====================================================================
        // Input:  ; select 1
        // Expect: ; SELECT 1
        // Actual: SELECT 1
        // Issue:  Leading semicolon (empty statement) is dropped
        {
            name: 'Leading semicolon should not be lost',
            input: '; select 1',
            expected: '; SELECT 1',
        },

        // =====================================================================
        // BUG #46: CLONE not uppercased (Delta Lake)
        // =====================================================================
        // Input:  create table t clone s
        // Expect: CREATE TABLE t CLONE s
        // Actual: CREATE TABLE t clone s
        // Issue:  CLONE keyword not uppercased
        {
            name: 'CLONE should be uppercased',
            input: 'create table t clone s',
            expected: 'CREATE TABLE t CLONE s',
        },

        // =====================================================================
        // BUG #47: SHALLOW CLONE not uppercased (Delta Lake)
        // =====================================================================
        // Input:  create table t shallow clone s
        // Expect: CREATE TABLE t SHALLOW CLONE s
        // Actual: CREATE TABLE t shallow clone s
        // Issue:  SHALLOW and CLONE keywords not uppercased
        {
            name: 'SHALLOW CLONE should be uppercased',
            input: 'create table t shallow clone s',
            expected: 'CREATE TABLE t SHALLOW CLONE s',
        },

        // =====================================================================
        // BUG #48: DEEP CLONE not uppercased (Delta Lake)
        // =====================================================================
        // Input:  create table t deep clone s
        // Expect: CREATE TABLE t DEEP CLONE s
        // Actual: CREATE TABLE t deep clone s
        // Issue:  DEEP and CLONE keywords not uppercased
        {
            name: 'DEEP CLONE should be uppercased',
            input: 'create table t deep clone s',
            expected: 'CREATE TABLE t DEEP CLONE s',
        },

        // =====================================================================
        // BUG #49: MAP_KEYS function not uppercased
        // =====================================================================
        // Input:  select map_keys(m) from t
        // Expect: SELECT MAP_KEYS(m) FROM t
        // Actual: SELECT map_keys(m) FROM t
        // Issue:  Function not uppercased
        {
            name: 'MAP_KEYS function should be uppercased',
            input: 'select map_keys(m) from t',
            expected: 'SELECT MAP_KEYS(m) FROM t',
        },

        // =====================================================================
        // BUG #50: JSON_OBJECT_KEYS function not uppercased
        // =====================================================================
        // Input:  select json_object_keys(j) from t
        // Expect: SELECT JSON_OBJECT_KEYS(j) FROM t
        // Actual: SELECT json_object_keys(j) FROM t
        // Issue:  Function not uppercased (available in Spark 3.1+)
        {
            name: 'JSON_OBJECT_KEYS function should be uppercased',
            input: 'select json_object_keys(j) from t',
            expected: 'SELECT JSON_OBJECT_KEYS(j) FROM t',
        },

        // =====================================================================
        // BUG #51: IF function not uppercased (Spark's IF function)
        // =====================================================================
        // Input:  select if(x > 0, 1, 0) from t
        // Expect: SELECT IF(x > 0, 1, 0) FROM t
        // Actual: SELECT if(x > 0, 1, 0) FROM t
        // Issue:  IF function not uppercased (it is a valid Spark function)
        {
            name: 'IF function should be uppercased',
            input: 'select if(x > 0, 1, 0) from t',
            expected: 'SELECT IF(x > 0, 1, 0) FROM t',
        },

        // =====================================================================
        // BUG #52: STRING_AGG function not uppercased
        // =====================================================================
        // Input:  select string_agg(x, ',') from t
        // Expect: SELECT STRING_AGG(x, ',') FROM t
        // Actual: SELECT string_agg(x, ',') FROM t
        // Issue:  Function not uppercased
        {
            name: 'STRING_AGG function should be uppercased',
            input: "select string_agg(x, ',') from t",
            expected: "SELECT STRING_AGG(x, ',') FROM t",
        },

        // =====================================================================
        // BUG #53: FLATTEN function not uppercased
        // =====================================================================
        // Input:  select flatten(arr) from t
        // Expect: SELECT FLATTEN(arr) FROM t
        // Actual: SELECT flatten(arr) FROM t
        // Issue:  Function not uppercased (available in Spark)
        {
            name: 'FLATTEN function should be uppercased',
            input: 'select flatten(arr) from t',
            expected: 'SELECT FLATTEN(arr) FROM t',
        },

        // =====================================================================
        // BUG #54: DISTRIBUTE BY not fully uppercased
        // =====================================================================
        // Input:  select * from t distribute by x
        // Expect: SELECT * FROM t DISTRIBUTE BY x
        // Actual: SELECT * FROM t distribute BY x
        // Issue:  DISTRIBUTE not uppercased
        {
            name: 'DISTRIBUTE BY should be fully uppercased',
            input: 'select * from t distribute by x',
            expected: 'SELECT * FROM t DISTRIBUTE BY x',
        },

        // =====================================================================
        // BUG #55: SORT BY not fully uppercased
        // =====================================================================
        // Input:  select * from t sort by x
        // Expect: SELECT * FROM t SORT BY x
        // Actual: SELECT * FROM t sort BY x
        // Issue:  SORT not uppercased
        {
            name: 'SORT BY should be fully uppercased',
            input: 'select * from t sort by x',
            expected: 'SELECT * FROM t SORT BY x',
        },

        // =====================================================================
        // BUG #56: CLUSTER BY not fully uppercased
        // =====================================================================
        // Input:  select * from t cluster by x
        // Expect: SELECT * FROM t CLUSTER BY x
        // Actual: SELECT * FROM t cluster BY x
        // Issue:  CLUSTER not uppercased
        {
            name: 'CLUSTER BY should be fully uppercased',
            input: 'select * from t cluster by x',
            expected: 'SELECT * FROM t CLUSTER BY x',
        },
    ],
};
