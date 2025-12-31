/**
 * sparkfmt - Spark SQL & Python Formatter
 * 
 * A unified formatter for Spark SQL and Python code, designed for
 * Microsoft Fabric notebooks and CI/CD pipelines.
 * 
 * Architecture:
 * - formatters/sql/: Core SQL formatting (ANTLR grammar-driven)
 * - formatters/python/: Python formatting via Ruff WASM
 * - notebook-formatter.ts: Fabric notebook handling
 */

// ============================================================================
// SQL Formatter (Core API)
// ============================================================================

export { formatSql, needsFormatting } from './formatters/sql/index.js';

// ============================================================================
// Language Formatters (Extensible)
// ============================================================================

export { 
    // Registry
    getFormatterRegistry,
    detectLanguage,
    
    // SQL
    SqlFormatter,
    getSqlFormatter,
    isSqlCode,
    
    // Python
    PythonFormatter,
    getPythonFormatter,
    isPythonCode,
    
    // Types
    type LanguageFormatter,
    type FormatterOptions,
    type FormatResult,
    type FormatterConfig,
    type FormatterRegistry,
} from './formatters/index.js';

// ============================================================================
// Notebook Formatter
// ============================================================================

export {
    // New API
    parseNotebook,
    formatNotebook,
    type NotebookCell,
    type FabricNotebook,
    type FormatStats,
    
    // Legacy API (deprecated but maintained for compatibility)
    extractMagicSqlCells,
    formatFabricNotebook,
    type MagicSqlCell,
    type MagicSqlFile,
} from './notebook-formatter.js';

// ============================================================================
// Configuration (Python/Ruff)
// ============================================================================

export {
    loadRuffConfig,
    toRuffWasmConfig,
    DEFAULT_RUFF_CONFIG,
    type RuffConfig,
    type RuffFormatConfig,
} from './formatters/python/index.js';

// ============================================================================
// Format Directives (SQL)
// ============================================================================

export { 
    hasFormatOff, 
    detectCollapseDirectives, 
    hasCollapseDirective, 
    type FormatDirectiveInfo 
} from './formatters/sql/index.js';

// ============================================================================
// Types (for library consumers)
// ============================================================================

export type { 
    AnalyzerResult,
    FormattingState,
    MultiArgFunctionInfo,
    WindowDefInfo,
    TokenContext,
    PendingComment,
    ExpandedFunction,
    ExpandedWindow
} from './formatters/sql/types.js';
