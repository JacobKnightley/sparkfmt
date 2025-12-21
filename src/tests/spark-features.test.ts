/**
 * Spark-Specific Feature Tests
 */
import { TestSuite } from './framework.js';

export const sparkFeaturesTests: TestSuite = {
    name: 'Spark-Specific Features',
    tests: [
        {
            name: 'LATERAL VIEW EXPLODE',
            input: 'select a, b from t lateral view explode(arr) AS item',
            expected: 'SELECT\n     a\n    ,b\nFROM t LATERAL VIEW EXPLODE(arr) AS item',
        },
        {
            name: 'WINDOW clause (named window)',
            input: 'select a, sum(b) over w from t window w as (partition by c order by d)',
            expected: 'SELECT\n     a\n    ,SUM(b) OVER w\nFROM t WINDOW w AS (PARTITION BY c ORDER BY d)',
        },
        {
            name: 'PIVOT',
            input: 'select * from t pivot (sum(val) for col in (1, 2, 3))',
            expected: 'SELECT *\nFROM t PIVOT (SUM(val) FOR col IN (1, 2, 3))',
        },
        {
            name: 'CLUSTER BY',
            input: 'select a, b from t cluster by a',
            expected: 'SELECT\n     a\n    ,b\nFROM t CLUSTER BY a',
        },
        {
            name: 'DISTRIBUTE BY SORT BY',
            input: 'select a, b from t distribute by a sort by b',
            expected: 'SELECT\n     a\n    ,b\nFROM t DISTRIBUTE BY a SORT BY b',
        },
        {
            name: 'TABLESAMPLE',
            input: 'select * from t tablesample (10 percent)',
            expected: 'SELECT *\nFROM t TABLESAMPLE (10 PERCENT)',
        },
    ],
};

export const lambdaTests: TestSuite = {
    name: 'Lambda Expressions',
    tests: [
        {
            name: 'Lambda expression (TRANSFORM)',
            input: 'select transform(arr, x -> x + 1) from t',
            expected: 'SELECT TRANSFORM(arr, x -> x + 1)\nFROM t',
        },
        {
            name: 'Lambda expression (FILTER)',
            input: 'select filter(arr, x -> x > 0) from t',
            expected: 'SELECT FILTER(arr, x -> x > 0)\nFROM t',
        },
        {
            name: 'Lambda expression (AGGREGATE)',
            input: 'select aggregate(arr, 0, (acc, x) -> acc + x) from t',
            expected: 'SELECT AGGREGATE(arr, 0, (acc, x) -> acc + x)\nFROM t',
        },
    ],
};
