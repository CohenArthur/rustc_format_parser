use rfp_ffi::rust;

fn main() {
    dbg!(rust::collect_pieces(
        std::env::args().nth(1).unwrap().as_str()
    ));
}
