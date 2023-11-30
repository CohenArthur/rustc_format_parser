//! FFI interface for `rustc_format_parser`

// what's the plan? Have a function return something that can be constructed into a vector?
// or an iterator?

use std::ffi::CStr;

pub mod rust;

use rustc_format_parser::Piece;

#[repr(C)]
pub struct PieceSlice {
    base_ptr: *const Piece<'static /* FIXME: That's wrong */>,
    len: usize,
}

#[no_mangle]
pub extern "C" fn collect_pieces(input: *const libc::c_char) -> PieceSlice {
    // FIXME: Add comment
    let str = unsafe { CStr::from_ptr(input) };

    // FIXME: No unwrap
    let pieces = rust::collect_pieces(str.to_str().unwrap());

    PieceSlice {
        base_ptr: pieces.as_ptr(),
        len: pieces.len(),
    }
}
