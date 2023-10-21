
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
