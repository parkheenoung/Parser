//! This crate can be used to parse Python source code into an Abstract
//! Syntax Tree.
//!
//! ## Overview:
//!
//! The process by which source code is parsed into an AST can be broken down
//! into two general stages: [lexical analysis] and [parsing].
//!
//! During lexical analysis, the source code is converted into a stream of lexical
//! tokens that represent the smallest meaningful units of the language. For example,
//! the source code `print("Hello world")` would _roughly_ be converted into the following
//! stream of tokens:
//!
//! ```text
//! Name("print"), LeftParen, String("Hello world"), RightParen
//! ```
//!
//! these tokens are then consumed by the parser, which matches them against a set of
//! grammar rules to verify that the source code is syntactically valid and to construct
//! an AST that represents the source code.
//!  
//! During parsing, the parser consumes the tokens generated by the lexer and constructs
//! a tree representation of the source code. The tree is made up of nodes that represent
//! the different syntactic constructs of the language. If the source code is syntactically
//! invalid, parsing fails and an error is returned. After a successful parse, the AST can
//! be used to perform further analysis on the source code. Continuing with the example
//! above, the AST generated by the parser would _roughly_ look something like this:
//!
//! ```text
//! node: Expr {
//!     value: {
//!         node: Call {
//!             func: {
//!                 node: Name {
//!                     id: "print",
//!                     ctx: Load,
//!                 },
//!             },
//!             args: [
//!                 node: Constant {
//!                     value: Str("Hello World"),
//!                     kind: None,
//!                 },
//!             ],
//!             keywords: [],
//!         },
//!     },
//! },
//!```
//!
//! Note: The Tokens/ASTs shown above are not the exact tokens/ASTs generated by the parser.
//!
//! ## Source code layout:
//!
//! The functionality of this crate is split into several modules:
//!
//! - [token]: This module contains the definition of the tokens that are generated by the lexer.
//! - [lexer]: This module contains the lexer and is responsible for generating the tokens.
//! - [parser]: This module contains an interface to the parser and is responsible for generating the AST.
//!     - Functions and strings have special parsing requirements that are handled in additional files.
//! - [mode]: This module contains the definition of the different modes that the parser can be in.
//! - [error]: This module contains the definition of the errors that can be returned by the parser.
//!
//! # Examples
//!
//! For example, to get a stream of tokens from a given string, one could do this:
//!
//! ```
//! use rustpython_parser::mode::Mode;
//! use rustpython_parser::lexer::make_tokenizer;
//!
//! let python_source = r#"
//! def is_odd(i):
//!     return bool(i & 1)
//! "#;
//! let mut tokens = make_tokenizer(python_source, Mode::Module);
//! assert!(tokens.all(|t| t.is_ok()));
//! ```
//!
//! These tokens can be directly fed into the parser to generate an AST:
//!
//! ```
//! use rustpython_parser::lexer::make_tokenizer;
//! use rustpython_parser::mode::Mode;
//! use rustpython_parser::parser::parse_tokens;
//!
//! let python_source = r#"
//! def is_odd(i):
//!    return bool(i & 1)
//! "#;
//! let tokens = make_tokenizer(python_source, Mode::Module);
//! let ast = parse_tokens(tokens, Mode::Module, "<embedded>");
//!
//! assert!(ast.is_ok());
//! ```
//!
//! Alternatively, you can use one of the other `parse_*` functions to parse a string directly without using a specific
//! mode or tokenizing the source beforehand:
//!
//! ```
//! use rustpython_parser::parser::parse_program;
//!
//! let python_source = r#"
//! def is_odd(i):
//!   return bool(i & 1)
//! "#;
//! let ast = parse_program(python_source, "<embedded>");
//!
//! assert!(ast.is_ok());
//! ```
//!
//! [lexical analysis]: https://en.wikipedia.org/wiki/Lexical_analysis
//! [parsing]: https://en.wikipedia.org/wiki/Parsing
//! [token]: crate::token
//! [lexer]: crate::lexer
//! [parser]: crate::parser
//! [mode]: crate::mode
//! [error]: crate::error

#![doc(html_logo_url = "https://raw.githubusercontent.com/RustPython/RustPython/main/logo.png")]
#![doc(html_root_url = "https://docs.rs/rustpython-parser/")]

#[macro_use]
extern crate log;
pub use rustpython_ast as ast;

mod function;
pub mod lexer;
pub mod mode;
pub mod parser;
mod string;
#[rustfmt::skip]
mod python;
mod context;
mod soft_keywords;
pub mod token;
