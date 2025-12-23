/**
 * Tests for magic-sql-extractor.ts
 * Tests extraction and formatting of SQL cells from Fabric notebook Python files
 */

import { TestSuite, TestCase } from './framework.js';
import { extractMagicSqlCells, addMagicPrefix, formatFabricNotebook } from '../magic-sql-extractor.js';
import { formatSql } from '../formatter.js';

// Helper: create a test that uses formatFabricNotebook
function fabricTest(name: string, input: string, expected: string): TestCase {
  return { name, input, expected };
}

const formatNotebookTests: TestCase[] = [
  fabricTest(
    'formats SQL cell in Fabric notebook (simple query stays inline)',
    `# Fabric notebook source

# CELL ********************

# MAGIC %%sql
# MAGIC select * from table where id=1

# METADATA ********************`,
    `# Fabric notebook source

# CELL ********************

# MAGIC %%sql
# MAGIC SELECT * FROM table WHERE id = 1

# METADATA ********************`
  ),
  fabricTest(
    'formats multiple SQL cells (simple queries stay inline)',
    `# Fabric notebook source

# CELL ********************

# MAGIC %%sql
# MAGIC select a from t1

# METADATA ********************

# CELL ********************

# MAGIC %%sql
# MAGIC select b from t2

# METADATA ********************`,
    `# Fabric notebook source

# CELL ********************

# MAGIC %%sql
# MAGIC SELECT a FROM t1

# METADATA ********************

# CELL ********************

# MAGIC %%sql
# MAGIC SELECT b FROM t2

# METADATA ********************`
  ),
  fabricTest(
    'preserves Python cells unchanged',
    `# Fabric notebook source

# CELL ********************

print("hello world")

# METADATA ********************

# CELL ********************

# MAGIC %%sql
# MAGIC select * from t

# METADATA ********************`,
    `# Fabric notebook source

# CELL ********************

print("hello world")

# METADATA ********************

# CELL ********************

# MAGIC %%sql
# MAGIC SELECT * FROM t

# METADATA ********************`
  ),
  fabricTest(
    'returns non-Fabric file unchanged',
    `# Regular Python file
print("hello")`,
    `# Regular Python file
print("hello")`
  ),
  fabricTest(
    'handles multi-line SQL with JOINs (expands)',
    `# Fabric notebook source

# CELL ********************

# MAGIC %%sql
# MAGIC select a.id, b.name from table_a a inner join table_b b on a.id = b.id where a.status = 1

# METADATA ********************`,
    `# Fabric notebook source

# CELL ********************

# MAGIC %%sql
# MAGIC SELECT
# MAGIC      a.id
# MAGIC     ,b.name
# MAGIC FROM table_a a
# MAGIC INNER JOIN table_b b
# MAGIC     ON a.id = b.id
# MAGIC WHERE a.status = 1

# METADATA ********************`
  ),
  fabricTest(
    'handles CREATE VIEW statements (expands with multiple columns)',
    `# Fabric notebook source

# CELL ********************

# MAGIC %%sql
# MAGIC create or replace temporary view vwTest as select col1, col2 from source_table

# METADATA ********************`,
    `# Fabric notebook source

# CELL ********************

# MAGIC %%sql
# MAGIC CREATE OR REPLACE TEMPORARY VIEW vwTest AS
# MAGIC SELECT
# MAGIC      col1
# MAGIC     ,col2
# MAGIC FROM source_table

# METADATA ********************`
  ),
];

// Custom format function that uses formatFabricNotebook
function formatFabricNotebookWrapper(input: string): string {
  return formatFabricNotebook(input, formatSql);
}

export const magicSqlSuite: TestSuite = {
  name: 'Magic SQL Extractor (Fabric Notebooks)',
  tests: formatNotebookTests,
};

// Export a custom runner for this suite since it uses formatFabricNotebook
export function runMagicSqlSuite(): { passed: number; failed: number; results: Array<{ name: string; passed: boolean; input?: string; expected?: string; got?: string }> } {
  const results: Array<{ name: string; passed: boolean; input?: string; expected?: string; got?: string }> = [];
  let passed = 0;
  let failed = 0;

  for (const tc of formatNotebookTests) {
    const result = formatFabricNotebookWrapper(tc.input);
    const success = result === tc.expected;

    if (success) {
      passed++;
      results.push({ name: tc.name, passed: true });
    } else {
      failed++;
      results.push({
        name: tc.name,
        passed: false,
        input: tc.input,
        expected: tc.expected,
        got: result,
      });
    }
  }

  return { passed, failed, results };
}
