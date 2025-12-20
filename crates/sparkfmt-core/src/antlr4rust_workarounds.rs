//! Workarounds for antlr4rust generator limitations
//!
//! # Purpose
//!
//! The antlr4rust code generator (v0.5.x) has a limitation where semantic
//! predicates in the grammar (e.g., `{method()}?`) generate code that calls
//! methods directly on the `recog` parameter:
//!
//! ```text
//! // Generated code calls:
//! recog.some_predicate_method()
//! ```
//!
//! Where `recog` is `&mut BaseLexer<..., Actions, ...>` or `&mut BaseParser<..., Ext, ...>`.
//!
//! The generated `Actions`/`Ext` structs are empty, and BaseLexer/BaseParser
//! implement `Deref<Target = Actions/Ext>`. This means the generated code
//! expects methods to exist on the Actions/Ext types.
//!
//! # Solution
//!
//! We use Rust extension traits to add methods to the Base types. For this to
//! work, the traits must be in scope where the generated code calls them.
//!
//! This module re-exports predicates in a way that makes them available to
//! the generated code.
//!
//! # Deletion Criteria
//!
//! This module can be deleted when antlr4rust provides one of:
//! 1. A mechanism to inject fields/methods into generated Actions/Ext structs
//! 2. A callback/trait-based predicate system
//! 3. Any other way to implement custom predicates without extension traits
//!
//! When deleting, also:
//! - Remove `mod antlr4rust_workarounds;` from lib.rs
//! - Remove `use crate::antlr4rust_workarounds::*;` from generated/mod.rs
//! - The predicates.rs module will continue to work unchanged
//!
//! # Related Issues
//!
//! - antlr4rust semantic predicate support: (no issue tracker link available)
//! - This workaround implemented: 2024-12-19

// Re-export all predicate traits so they're in scope for generated code.
// The generated code calls methods like `recog.is_valid_decimal()` which
// Rust resolves via these extension traits.
pub use crate::antlr_predicates::{LexerPredicates, ParserPredicates};
