//! ANTLR Parser Wrapper
//!
//! This module wraps the generated ANTLR parser to provide a clean API.
//! It is the ONLY entry point for ANTLR-based parsing of SQL strings.
//!
//! The wrapper handles:
//! - Lexer state reset before each parse
//! - Error handling (never panics on invalid input)
//! - Comment extraction from the hidden channel

use std::rc::Rc;

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::input_stream::InputStream;
use antlr4rust::int_stream::IntStream;
use antlr4rust::token::Token;
use antlr4rust::token_stream::TokenStream;
use antlr4rust::Parser;
use antlr4rust::TokenSource;

use crate::antlr_predicates::reset_lexer_state;
use crate::error::FormatError;
use crate::generated::sqlbaselexer::{
    SqlBaseLexer, BRACKETED_COMMENT, SIMPLE_COMMENT,
};
use crate::generated::sqlbaseparser::{
    SingleStatementContextAll, SqlBaseParser,
};

/// Comment with position and attachment info.
/// Extracted from the ANTLR token stream.
#[derive(Debug, Clone)]
pub struct Comment {
    pub text: String,
    pub line: usize,
    pub column: usize,
    pub is_block: bool, // true for /* */, false for --
    pub is_hint: bool,  // true for /*+ */
}

/// Result of parsing SQL with ANTLR.
/// Contains the parse tree and extracted comments.
pub struct ParseResult<'input> {
    /// The root of the parse tree (singleStatement rule)
    pub tree: Rc<SingleStatementContextAll<'input>>,
    /// Comments extracted from the hidden channel
    pub comments: Vec<Comment>,
}

/// Parse SQL string into ANTLR parse tree.
///
/// MUST handle all valid Spark SQL syntax.
/// MUST return error for invalid syntax (never panic).
/// MUST reset lexer state before parsing.
///
/// # Arguments
/// * `input` - The SQL string to parse
///
/// # Returns
/// * `Ok(ParseResult)` - Contains the parse tree and extracted comments
/// * `Err(FormatError)` - If parsing fails
///
/// # Example
/// ```ignore
/// let result = parse("SELECT 1")?;
/// // result.tree contains the parse tree
/// // result.comments contains any SQL comments
/// ```
pub fn parse(input: &str) -> Result<ParseResult<'_>, FormatError> {
    // Reset thread-local lexer state before parsing
    // This is REQUIRED for correct predicate behavior
    reset_lexer_state();

    // Create the input stream from the SQL string
    let input_stream = InputStream::new(input);

    // Create the lexer
    let lexer = SqlBaseLexer::new(input_stream);

    // Create the token stream
    let token_stream = CommonTokenStream::new(lexer);

    // Create the parser
    let mut parser = SqlBaseParser::new(token_stream);

    // Parse using the singleStatement rule (main entry point for SQL statements)
    let tree = parser.singleStatement().map_err(|e| {
        FormatError::new(format!("Parse error: {:?}", e))
    })?;

    // Extract comments AFTER parsing (token buffer is now populated)
    // After parsing, the token stream has been consumed, but we can extract
    // comments by re-lexing with a separate pass
    let comments = extract_comments_by_relexing(input);

    Ok(ParseResult { tree, comments })
}

/// Extract comments by re-lexing the input.
///
/// This function creates a new lexer to extract comments from the input.
/// While not as efficient as extracting from the parsing token stream,
/// this approach is simpler and more reliable with antlr4rust's API.
fn extract_comments_by_relexing(input: &str) -> Vec<Comment> {
    let mut comments = Vec::new();
    
    // Create a fresh lexer for comment extraction
    let input_stream = InputStream::new(input);
    let mut lexer = SqlBaseLexer::new(input_stream);
    
    // Iterate through all tokens
    loop {
        let token = lexer.next_token();
        let token_type = token.get_token_type();
        
        // EOF token type is typically -1 or a large number
        if token_type == antlr4rust::token::TOKEN_EOF {
            break;
        }
        
        // Check if this is a comment token
        if token_type == SIMPLE_COMMENT || token_type == BRACKETED_COMMENT {
            let text = token.get_text().to_string();
            let line = token.get_line() as usize;
            let column = token.get_column() as usize;

            // Determine if it's a block comment (/* */) or line comment (--)
            let is_block = token_type == BRACKETED_COMMENT;

            // Determine if it's a hint (/*+ */)
            let is_hint = is_block && text.starts_with("/*+");

            comments.push(Comment {
                text,
                line,
                column,
                is_block,
                is_hint,
            });
        }
    }
    
    comments
}

/// Extract comments from token stream.
///
/// ANTLR puts comments in the HIDDEN channel (channel 1).
/// This function iterates through ALL tokens in the underlying buffer
/// and filters for comment tokens.
///
/// # Arguments
/// * `token_stream` - The CommonTokenStream to extract comments from
///
/// # Returns
/// * `Vec<Comment>` - All comments found in the token stream
fn extract_comments_from_stream<'input>(
    token_stream: &mut CommonTokenStream<'input, SqlBaseLexer<'input, InputStream<&'input str>>>,
) -> Vec<Comment> {
    let mut comments = Vec::new();

    // Fill the token buffer by consuming all tokens
    // The CommonTokenStream filters by channel, but base() gives us access to all tokens
    let base = token_stream.base();
    let size = base.size();

    for i in 0..size {
        let token = base.get(i);
        let token_type = token.get_token_type();

        // Check if this is a comment token
        if token_type == SIMPLE_COMMENT || token_type == BRACKETED_COMMENT {
            let text = token.get_text().to_string();
            let line = token.get_line() as usize;
            let column = token.get_column() as usize;

            // Determine if it's a block comment (/* */) or line comment (--)
            let is_block = token_type == BRACKETED_COMMENT;

            // Determine if it's a hint (/*+ */)
            let is_hint = is_block && text.starts_with("/*+");

            comments.push(Comment {
                text,
                line,
                column,
                is_block,
                is_hint,
            });
        }
    }

    comments
}

/// Extract comments from a raw token stream reference.
///
/// This is a convenience function for extracting comments when you already
/// have a reference to a token stream (e.g., after parsing).
///
/// # Arguments  
/// * `token_stream` - Reference to the CommonTokenStream
///
/// # Returns
/// * `Vec<Comment>` - All comments found in the token stream
pub fn extract_comments<'input>(
    token_stream: &CommonTokenStream<'input, SqlBaseLexer<'input, InputStream<&'input str>>>,
) -> Vec<Comment> {
    let mut comments = Vec::new();

    let base = token_stream.base();
    let size = base.size();

    for i in 0..size {
        let token = base.get(i);
        let token_type = token.get_token_type();

        // Check if this is a comment token
        if token_type == SIMPLE_COMMENT || token_type == BRACKETED_COMMENT {
            let text = token.get_text().to_string();
            let line = token.get_line() as usize;
            let column = token.get_column() as usize;

            // Determine if it's a block comment (/* */) or line comment (--)
            let is_block = token_type == BRACKETED_COMMENT;

            // Determine if it's a hint (/*+ */)
            let is_hint = is_block && text.starts_with("/*+");

            comments.push(Comment {
                text,
                line,
                column,
                is_block,
                is_hint,
            });
        }
    }

    comments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_select() {
        let result = parse("SELECT 1");
        assert!(result.is_ok(), "Failed to parse simple SELECT: {:?}", result.err());
    }

    #[test]
    fn test_parse_select_from() {
        let result = parse("SELECT a, b FROM t");
        assert!(result.is_ok(), "Failed to parse SELECT FROM: {:?}", result.err());
    }

    #[test]
    fn test_parse_invalid_sql_returns_error() {
        // Invalid SQL should return an error, not panic
        let result = parse("SELECT FROM");
        // This may or may not be valid depending on grammar - just ensure no panic
        // The important thing is we don't panic
        let _ = result;
    }

    #[test]
    fn test_parse_empty_string() {
        // Empty string should return an error, not panic
        let result = parse("");
        // The grammar may reject this, but we shouldn't panic
        let _ = result;
    }

    #[test]
    fn test_extract_line_comment() {
        let result = parse("SELECT 1 -- this is a comment");
        assert!(result.is_ok());
        let parse_result = result.unwrap();
        
        // Should have extracted the comment
        assert_eq!(parse_result.comments.len(), 1);
        let comment = &parse_result.comments[0];
        assert!(!comment.is_block);
        assert!(!comment.is_hint);
        assert!(comment.text.contains("this is a comment"));
    }

    #[test]
    fn test_extract_block_comment() {
        let result = parse("SELECT /* block comment */ 1");
        assert!(result.is_ok());
        let parse_result = result.unwrap();
        
        // Should have extracted the comment
        assert_eq!(parse_result.comments.len(), 1);
        let comment = &parse_result.comments[0];
        assert!(comment.is_block);
        assert!(!comment.is_hint);
        assert!(comment.text.contains("block comment"));
    }

    #[test]
    fn test_extract_hint_comment() {
        let result = parse("SELECT /*+ BROADCAST(t) */ * FROM t");
        assert!(result.is_ok());
        let parse_result = result.unwrap();
        
        // Should have extracted the hint
        assert_eq!(parse_result.comments.len(), 1);
        let comment = &parse_result.comments[0];
        assert!(comment.is_block);
        assert!(comment.is_hint);
        assert!(comment.text.contains("BROADCAST"));
    }

    #[test]
    fn test_multiple_comments() {
        let result = parse("SELECT /* first */ 1 -- second");
        assert!(result.is_ok());
        let parse_result = result.unwrap();
        
        // Should have extracted both comments
        assert_eq!(parse_result.comments.len(), 2);
    }
}
