
// run: `cargo clippy`
//
// warning: manual `Range::contains` implementation
// --> src\cool_part\smart_compiler.rs:4:5
// |
// 4 |     i >= 10 && i < 20
// |     ^^^^^^^^^^^^^^^^^ help: use: `(10..20).contains(&i)`
// |
// = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#manual_range_contains
// = note: `#[warn(clippy::manual_range_contains)]` on by default
//
// all clippy lints: https://rust-lang.github.io/rust-clippy/master/index.html
#[allow(dead_code)]
fn in_range(i: usize) -> bool {
    i >= 10 && i < 20
}

// warning: match expression looks like `matches!` macro
//   --> src/asn1/hex_view.rs:52:5
//    |
// 52 | /     match cur_node {
// 53 | |         Some(node_id) if *node_id == asn1_node_id => true,
// 54 | |         _ => false,
// 55 | |     }
//    | |_____^ help: try: `matches!(cur_node, Some(node_id) if *node_id == asn1_node_id)`
//    |
//    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#match_like_matches_macro
//    = note: `#[warn(clippy::match_like_matches_macro)]` on by default
fn compare_ids(asn1_node_id: u64, cur_node: &Option<u64>) -> bool {
    match cur_node {
        Some(node_id) if *node_id == asn1_node_id => true,
        _ => false,
    }
}
