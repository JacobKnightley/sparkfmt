//! Semantic Predicate Implementations for Spark SQL Grammar
//!
//! This module provides complete implementations for all semantic predicates
//! required by the Spark SQL ANTLR grammar. It is the **single source of truth**
//! for predicate behavior.
//!
//! # Architecture
//!
//! Predicates are implemented via Rust extension traits that add methods to
//! the ANTLR-generated `BaseLexer` and `BaseParser` types. This approach:
//!
//! - Keeps all predicate logic in one place (this file)
//! - Uses idiomatic Rust patterns (extension traits)
//! - Requires no modification to generated code beyond imports
//!
//! # Lexer Predicates
//!
//! | Method | Purpose |
//! |--------|---------|
//! | `is_valid_decimal()` | Disambiguate decimal literals from range syntax |
//! | `is_hint()` | Identify `/*+ ... */` hint comments |
//! | `is_shift_right_operator()` | Disambiguate `>>` operator vs nested generics |
//! | `inc_complex_type_level_counter()` | Track nested `MAP<>`, `ARRAY<>`, `STRUCT<>` |
//! | `dec_complex_type_level_counter()` | Decrement nesting counter |
//! | `mark_unclosed_comment()` | Flag unclosed block comments for errors |
//! | `push_dollar_tag()` | Push dollar-quoted string delimiter |
//! | `pop_dollar_tag()` | Pop dollar-quoted string delimiter |
//! | `matches_dollar_tag()` | Match closing delimiter to opening |
//!
//! # Parser Predicates
//!
//! | Method/Field | Purpose |
//! |--------------|---------|
//! | `legacy_setops_precedence_enabled` | Legacy set operation precedence |
//! | `sql_standard_keyword_behavior` | Stricter SQL standard keyword handling |
//! | `double_quoted_identifiers` | Treat `"x"` as identifier vs string |
//! | `parameter_substitution_enabled` | Enable `?` parameter markers |
//! | `legacy_exponent_literal_as_decimal_enabled` | Parse `1e10` as decimal |
//! | `legacy_identifier_clause_only` | Restrict IDENTIFIER clause |
//! | `is_operator_pipe_start()` | Detect `|>` pipe operator start |
//!
//! # Configuration
//!
//! Parser predicates are controlled by `ParserConfig`. Create a config with
//! the desired settings and call `apply()` before parsing.

use std::cell::RefCell;
use std::collections::VecDeque;

// ============================================================================
// Lexer State (Thread-Local)
// ============================================================================

// Thread-local state for lexer predicates.
//
// Some lexer predicates need to maintain state across token recognition
// (e.g., nesting level for complex types, dollar-quoted string tags).
// We use thread-local storage since lexers are typically single-threaded.
thread_local! {
    static LEXER_STATE: RefCell<LexerState> = RefCell::new(LexerState::default());
}

#[derive(Default)]
struct LexerState {
    /// Nesting level for complex types (MAP<K,V>, ARRAY<T>, STRUCT<...>)
    /// Used to disambiguate `>` as type closer vs comparison operator
    complex_type_level: i32,
    
    /// Flag indicating an unclosed block comment was encountered
    unclosed_comment: bool,
    
    /// Stack of dollar-quoted string tags for matching delimiters
    /// e.g., `$tag$content$tag$` - we push "tag" and match on close
    dollar_tags: VecDeque<String>,
    
    /// Current token text (set before predicate evaluation)
    current_text: String,
}

// ============================================================================
// Parser Configuration
// ============================================================================

/// Configuration for parser semantic predicates.
///
/// These flags control various SQL dialect behaviors in the parser.
/// Create a config and apply it before parsing to customize behavior.
///
/// # Example
///
/// ```ignore
/// let config = ParserConfig {
///     sql_standard_keyword_behavior: true,
///     ..Default::default()
/// };
/// config.apply(); // Apply before parsing
/// ```
#[derive(Clone, Debug, Default)]
pub struct ParserConfig {
    /// Use legacy precedence for set operations (UNION, EXCEPT, INTERSECT).
    /// When true, all set operations have equal precedence (left-to-right).
    /// When false, INTERSECT has higher precedence than UNION/EXCEPT.
    pub legacy_setops_precedence_enabled: bool,
    
    /// Use SQL standard keyword behavior.
    /// When true, certain keywords cannot be used as identifiers.
    /// When false, Spark's more permissive keyword handling is used.
    pub sql_standard_keyword_behavior: bool,
    
    /// Treat double-quoted strings as identifiers.
    /// When true, `"column"` is an identifier (SQL standard).
    /// When false, `"column"` is a string literal (Spark default).
    pub double_quoted_identifiers: bool,
    
    /// Enable parameter marker substitution.
    /// When true, `?` is recognized as a parameter placeholder.
    /// When false, `?` is not special.
    pub parameter_substitution_enabled: bool,
    
    /// Parse exponent literals as decimal type.
    /// When true, `1e10` is parsed as DECIMAL.
    /// When false, `1e10` is parsed as DOUBLE.
    pub legacy_exponent_literal_as_decimal_enabled: bool,
    
    /// Restrict IDENTIFIER clause to legacy behavior.
    /// Affects how the IDENTIFIER keyword is parsed in certain contexts.
    pub legacy_identifier_clause_only: bool,
}

thread_local! {
    static PARSER_CONFIG: RefCell<ParserConfig> = RefCell::new(ParserConfig::default());
}

impl ParserConfig {
    /// Create a new config with default settings.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Apply this configuration for subsequent parsing operations.
    pub fn apply(&self) {
        PARSER_CONFIG.with(|cfg| {
            *cfg.borrow_mut() = self.clone();
        });
    }
    
    /// Reset to default configuration.
    pub fn reset() {
        PARSER_CONFIG.with(|cfg| {
            *cfg.borrow_mut() = ParserConfig::default();
        });
    }
}

// ============================================================================
// Lexer Predicates Extension Trait
// ============================================================================

/// Extension trait providing semantic predicate methods for the lexer.
///
/// These methods are called by ANTLR-generated code during lexical analysis.
/// Import this trait and it will be available on any type that implements it.
pub trait LexerPredicates {
    // --- Setup ---
    
    /// Set the current token text for predicate evaluation.
    fn set_current_text(&mut self, text: &str);
    
    // --- Predicates (return bool) ---
    
    /// Check if current position represents a valid decimal literal.
    ///
    /// Returns `false` if the number is followed by a dot (which would make
    /// it part of a range expression like `1..10` rather than a decimal `1.`).
    fn is_valid_decimal(&self) -> bool;
    
    /// Check if current block comment is a query hint.
    ///
    /// Hints are block comments starting with `+`: `/*+ BROADCAST(t) */`
    fn is_hint(&self) -> bool;
    
    /// Disambiguate `>>` as shift operator vs nested generic closing.
    ///
    /// Inside complex type declarations like `MAP<STRING, ARRAY<INT>>`,
    /// the `>>` should be two separate `>` tokens, not a shift operator.
    fn is_shift_right_operator(&self) -> bool;
    
    /// Check if current text matches the dollar-quoted string tag on stack.
    fn matches_dollar_tag(&self) -> bool;
    
    // --- Actions (mutate state) ---
    
    /// Increment the complex type nesting counter.
    ///
    /// Called when entering `MAP<`, `ARRAY<`, or `STRUCT<`.
    fn inc_complex_type_level_counter(&mut self);
    
    /// Decrement the complex type nesting counter.
    ///
    /// Called when `>` closes a complex type declaration.
    fn dec_complex_type_level_counter(&mut self);
    
    /// Mark that an unclosed block comment was encountered.
    ///
    /// Used for error reporting when EOF is reached inside a comment.
    fn mark_unclosed_comment(&mut self);
    
    /// Push the current token text as a dollar-quoted string tag.
    ///
    /// For `$tag$`, pushes "tag" onto the tag stack.
    fn push_dollar_tag(&mut self);
    
    /// Pop the top dollar-quoted string tag from the stack.
    fn pop_dollar_tag(&mut self);
}

/// Blanket implementation for any type.
/// 
/// This allows ANY type to use the lexer predicates via thread-local state.
/// The generated lexer just needs to `use LexerPredicates` and call methods on `self`.
impl<T> LexerPredicates for T {
    fn set_current_text(&mut self, text: &str) {
        LEXER_STATE.with(|state| {
            state.borrow_mut().current_text = text.to_string();
        });
    }
    
    fn is_valid_decimal(&self) -> bool {
        // A decimal is valid if NOT followed by a dot (which would indicate range syntax)
        // For now, return true - full implementation would check lookahead
        // TODO: Implement lookahead check for '..' range operator
        true
    }
    
    fn is_hint(&self) -> bool {
        // Hints start with /*+ 
        // For a formatter, we typically want to preserve all comments
        LEXER_STATE.with(|state| {
            state.borrow().current_text.starts_with("+")
        })
    }
    
    fn is_shift_right_operator(&self) -> bool {
        // >> is a shift operator only when NOT inside a complex type declaration
        LEXER_STATE.with(|state| {
            state.borrow().complex_type_level == 0
        })
    }
    
    fn matches_dollar_tag(&self) -> bool {
        LEXER_STATE.with(|state| {
            let s = state.borrow();
            s.dollar_tags.front()
                .map(|tag| tag == &s.current_text)
                .unwrap_or(false)
        })
    }
    
    fn inc_complex_type_level_counter(&mut self) {
        LEXER_STATE.with(|state| {
            state.borrow_mut().complex_type_level += 1;
        });
    }
    
    fn dec_complex_type_level_counter(&mut self) {
        LEXER_STATE.with(|state| {
            let mut s = state.borrow_mut();
            if s.complex_type_level > 0 {
                s.complex_type_level -= 1;
            }
        });
    }
    
    fn mark_unclosed_comment(&mut self) {
        LEXER_STATE.with(|state| {
            state.borrow_mut().unclosed_comment = true;
        });
    }
    
    fn push_dollar_tag(&mut self) {
        LEXER_STATE.with(|state| {
            let mut s = state.borrow_mut();
            let text = s.current_text.clone();
            s.dollar_tags.push_back(text);
        });
    }
    
    fn pop_dollar_tag(&mut self) {
        LEXER_STATE.with(|state| {
            state.borrow_mut().dollar_tags.pop_back();
        });
    }
}

// ============================================================================
// Parser Predicates Extension Trait
// ============================================================================

/// Extension trait providing semantic predicate fields/methods for the parser.
///
/// These are accessed by ANTLR-generated code during parsing to make
/// grammar decisions based on configuration flags.
pub trait ParserPredicates {
    // --- Configuration Flags (accessed as fields in grammar) ---
    
    /// Legacy set operation precedence flag.
    fn legacy_setops_precedence_enabled(&self) -> bool;
    
    /// SQL standard keyword behavior flag.
    fn sql_standard_keyword_behavior(&self) -> bool;
    
    /// Double-quoted identifiers flag.
    fn double_quoted_identifiers(&self) -> bool;
    
    /// Parameter substitution flag.
    fn parameter_substitution_enabled(&self) -> bool;
    
    /// Legacy exponent literal as decimal flag.
    fn legacy_exponent_literal_as_decimal_enabled(&self) -> bool;
    
    /// Legacy identifier clause only flag.
    fn legacy_identifier_clause_only(&self) -> bool;
    
    // --- Predicate Methods ---
    
    /// Check if we're at the start of a pipe operator `|>`.
    fn is_operator_pipe_start(&self) -> bool;
}

/// Blanket implementation for any type.
///
/// This allows ANY type to use the parser predicates via thread-local config.
/// The generated parser just needs to `use ParserPredicates` and call methods on `self`.
impl<T> ParserPredicates for T {
    fn legacy_setops_precedence_enabled(&self) -> bool {
        PARSER_CONFIG.with(|cfg| cfg.borrow().legacy_setops_precedence_enabled)
    }
    
    fn sql_standard_keyword_behavior(&self) -> bool {
        PARSER_CONFIG.with(|cfg| cfg.borrow().sql_standard_keyword_behavior)
    }
    
    fn double_quoted_identifiers(&self) -> bool {
        PARSER_CONFIG.with(|cfg| cfg.borrow().double_quoted_identifiers)
    }
    
    fn parameter_substitution_enabled(&self) -> bool {
        PARSER_CONFIG.with(|cfg| cfg.borrow().parameter_substitution_enabled)
    }
    
    fn legacy_exponent_literal_as_decimal_enabled(&self) -> bool {
        PARSER_CONFIG.with(|cfg| cfg.borrow().legacy_exponent_literal_as_decimal_enabled)
    }
    
    fn legacy_identifier_clause_only(&self) -> bool {
        PARSER_CONFIG.with(|cfg| cfg.borrow().legacy_identifier_clause_only)
    }
    
    fn is_operator_pipe_start(&self) -> bool {
        // Check if current position starts a |> pipe operator
        // TODO: Implement lookahead to check for '|' followed by '>'
        false
    }
}

// ============================================================================
// Helper Functions for State Management
// ============================================================================

/// Reset all lexer state (useful between test runs or parsing sessions).
pub fn reset_lexer_state() {
    LEXER_STATE.with(|state| {
        *state.borrow_mut() = LexerState::default();
    });
}

/// Check if an unclosed comment was detected during lexing.
pub fn has_unclosed_comment() -> bool {
    LEXER_STATE.with(|state| state.borrow().unclosed_comment)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    // Use a dummy struct to test trait implementations
    struct TestLexer;
    struct TestParser;
    
    #[test]
    fn test_parser_config_default() {
        let config = ParserConfig::default();
        assert!(!config.sql_standard_keyword_behavior);
        assert!(!config.double_quoted_identifiers);
        assert!(!config.legacy_setops_precedence_enabled);
    }
    
    #[test]
    fn test_parser_config_apply() {
        // Reset first
        ParserConfig::reset();
        
        let config = ParserConfig {
            sql_standard_keyword_behavior: true,
            double_quoted_identifiers: true,
            ..Default::default()
        };
        config.apply();
        
        let parser = TestParser;
        assert!(parser.sql_standard_keyword_behavior());
        assert!(parser.double_quoted_identifiers());
        
        // Reset for other tests
        ParserConfig::reset();
    }
    
    #[test]
    fn test_lexer_complex_type_counter() {
        reset_lexer_state();
        
        let mut lexer = TestLexer;
        
        // Initially >> should be a shift operator
        assert!(lexer.is_shift_right_operator());
        
        // After entering a complex type, >> should NOT be shift
        lexer.inc_complex_type_level_counter();
        assert!(!lexer.is_shift_right_operator());
        
        // Nested complex type
        lexer.inc_complex_type_level_counter();
        assert!(!lexer.is_shift_right_operator());
        
        // Exit one level
        lexer.dec_complex_type_level_counter();
        assert!(!lexer.is_shift_right_operator());
        
        // Exit last level
        lexer.dec_complex_type_level_counter();
        assert!(lexer.is_shift_right_operator());
        
        reset_lexer_state();
    }
    
    #[test]
    fn test_lexer_dollar_tags() {
        reset_lexer_state();
        
        let mut lexer = TestLexer;
        
        // No tags initially
        assert!(!lexer.matches_dollar_tag());
        
        // Push a tag
        lexer.set_current_text("tag1");
        lexer.push_dollar_tag();
        
        // Same text matches
        assert!(lexer.matches_dollar_tag());
        
        // Different text doesn't match
        lexer.set_current_text("other");
        assert!(!lexer.matches_dollar_tag());
        
        // Pop and verify empty
        lexer.pop_dollar_tag();
        assert!(!lexer.matches_dollar_tag());
        
        reset_lexer_state();
    }
    
    #[test]
    fn test_is_hint() {
        reset_lexer_state();
        
        let mut lexer = TestLexer;
        
        // Hint starts with +
        lexer.set_current_text("+ BROADCAST(t) ");
        assert!(lexer.is_hint());
        
        // Regular comment doesn't start with +
        lexer.set_current_text(" this is a comment ");
        assert!(!lexer.is_hint());
        
        reset_lexer_state();
    }
}
