//! Rust functions to interact with `rustc_format_parser` from the FFI side of things

use rustc_format_parser::{ParseMode, Parser, Piece};

pub fn collect_pieces(input: &str) -> Vec<Piece<'_>> {
    // let parser = Parser::new();
    let parser = Parser::new(input, None, None, true, ParseMode::Format);

    parser.into_iter().collect()
}
