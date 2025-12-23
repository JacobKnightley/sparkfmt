/**
 * Magic SQL Extractor
 *
 * Handles Microsoft Fabric notebook Python files where SQL cells are stored
 * with `# MAGIC` line prefixes. These files are saved in source control as .py
 * files but contain embedded SQL cells.
 *
 * Format:
 * ```
 * # CELL ********************
 *
 * # MAGIC %%sql
 * # MAGIC SELECT *
 * # MAGIC FROM table
 *
 * # METADATA ********************
 *
 * # META {
 * # META   "language": "sparksql",
 * # META   "language_group": "synapse_pyspark"
 * # META }
 * ```
 */

export interface MagicSqlCell {
  /** Line index (0-based) where the cell content starts (after # CELL marker) */
  contentStartLine: number;
  /** Line index (0-based) where the cell content ends (before # METADATA) */
  contentEndLine: number;
  /** The extracted SQL without # MAGIC prefixes (excludes %%sql magic command) */
  sql: string;
  /** Original lines including # MAGIC prefixes (for reconstruction reference) */
  originalLines: string[];
}

export interface MagicSqlFile {
  /** Whether this file is a Fabric notebook (starts with "# Fabric notebook source") */
  isFabricNotebook: boolean;
  /** All detected SQL cells */
  sqlCells: MagicSqlCell[];
  /** Original file content split into lines */
  lines: string[];
}

const FABRIC_HEADER = '# Fabric notebook source';
const CELL_MARKER = '# CELL ********************';
const METADATA_MARKER = '# METADATA ********************';
const MAGIC_PREFIX = '# MAGIC ';
const MAGIC_SQL_COMMAND = '# MAGIC %%sql';

/**
 * Detect the line ending style used in content.
 */
function detectLineEnding(content: string): string {
  if (content.includes('\r\n')) {
    return '\r\n';
  }
  return '\n';
}

/**
 * Detects if a file is a Fabric notebook and extracts all SQL cells.
 */
export function extractMagicSqlCells(content: string): MagicSqlFile {
  const lines = content.split(/\r?\n/);

  const result: MagicSqlFile = {
    isFabricNotebook: false,
    sqlCells: [],
    lines,
  };

  // Check if this is a Fabric notebook
  if (lines.length === 0 || !lines[0].trim().startsWith(FABRIC_HEADER)) {
    return result;
  }

  result.isFabricNotebook = true;

  // Find all SQL cells
  let i = 0;
  while (i < lines.length) {
    // Look for CELL marker
    if (lines[i].trim() === CELL_MARKER) {
      const cellStartLine = i + 1;

      // Skip empty lines after CELL marker
      let j = cellStartLine;
      while (j < lines.length && lines[j].trim() === '') {
        j++;
      }

      // Check if this cell starts with %%sql magic
      if (j < lines.length && lines[j].trim() === MAGIC_SQL_COMMAND) {
        // This is a SQL cell - find its end
        const sqlStartLine = j + 1; // Line after %%sql
        let cellEndLine = sqlStartLine;

        // Find the end of the cell (next METADATA or CELL marker)
        while (cellEndLine < lines.length) {
          const trimmed = lines[cellEndLine].trim();
          if (trimmed === METADATA_MARKER || trimmed === CELL_MARKER) {
            break;
          }
          cellEndLine++;
        }

        // Back up past any trailing empty lines
        let actualEndLine = cellEndLine - 1;
        while (actualEndLine >= sqlStartLine && lines[actualEndLine].trim() === '') {
          actualEndLine--;
        }

        if (actualEndLine >= sqlStartLine) {
          // Extract SQL lines (strip # MAGIC prefix)
          const originalLines = lines.slice(sqlStartLine, actualEndLine + 1);
          const sqlLines = originalLines.map(line => stripMagicPrefix(line));

          result.sqlCells.push({
            contentStartLine: sqlStartLine,
            contentEndLine: actualEndLine,
            sql: sqlLines.join('\n'),
            originalLines,
          });
        }

        i = cellEndLine;
        continue;
      }
    }
    i++;
  }

  return result;
}

/**
 * Strips the # MAGIC prefix from a line.
 * Handles both "# MAGIC content" and "# MAGIC" (empty line).
 */
function stripMagicPrefix(line: string): string {
  if (line.startsWith(MAGIC_PREFIX)) {
    return line.slice(MAGIC_PREFIX.length);
  }
  if (line.trim() === '# MAGIC') {
    return '';
  }
  // Line doesn't have MAGIC prefix - return as-is (shouldn't happen in valid cells)
  return line;
}

/**
 * Adds the # MAGIC prefix to each line of SQL.
 */
export function addMagicPrefix(sql: string): string[] {
  return sql.split(/\r?\n/).map(line => {
    if (line === '') {
      return '# MAGIC';
    }
    return MAGIC_PREFIX + line;
  });
}

/**
 * Replaces a SQL cell's content with formatted SQL.
 * Returns the updated file content.
 */
export function replaceSqlCell(
  fileContent: string,
  cell: MagicSqlCell,
  formattedSql: string,
  lineEnding: string = '\n'
): string {
  const lines = fileContent.split(/\r?\n/);
  const newSqlLines = addMagicPrefix(formattedSql);

  // Replace the lines in the cell
  const before = lines.slice(0, cell.contentStartLine);
  const after = lines.slice(cell.contentEndLine + 1);

  const newLines = [...before, ...newSqlLines, ...after];
  return newLines.join(lineEnding);
}

/**
 * Formats all SQL cells in a Fabric notebook file.
 * @param content The file content
 * @param formatFn A function that formats SQL (e.g., formatSql from formatter.ts)
 * @returns The updated file content with formatted SQL cells
 */
export function formatFabricNotebook(
  content: string,
  formatFn: (sql: string) => string
): string {
  const extracted = extractMagicSqlCells(content);

  if (!extracted.isFabricNotebook || extracted.sqlCells.length === 0) {
    return content;
  }

  // Detect and preserve original line endings
  const lineEnding = detectLineEnding(content);

  // Process cells in reverse order so line numbers remain valid
  let result = content;
  const cellsReversed = [...extracted.sqlCells].reverse();

  for (const cell of cellsReversed) {
    const formattedSql = formatFn(cell.sql);
    result = replaceSqlCell(result, cell, formattedSql, lineEnding);
  }

  return result;
}
