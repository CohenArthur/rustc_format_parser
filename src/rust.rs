//! Rust functions to interact with `rustc_format_parser` from the FFI side of things

// TODO: Use rustc's version here #3
use generic_rustc_format_parser::{ParseMode, Parser, Piece};

pub fn collect_pieces(input: &str) -> Vec<Piece<'_>> {
    // let parser = Parser::new();
    let parser = Parser::new(input, None, None, true, ParseMode::Format);

    parser.into_iter().collect()
}
