use rfp_ffi::collect_pieces;

fn main() {
    dbg!(collect_pieces(std::env::args().nth(1).unwrap().as_str()));
}
