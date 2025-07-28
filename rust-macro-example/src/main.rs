use btree_proc_macro::btree_2;

#[macro_use]
mod declarative;

fn main() {
    // Declarative macro.
    let map_1 = btree_1! {
        1 => "tbt", 2 => "TheBestTvarynka"
    };
    println!("{map_1:?}");

    // Procedural macro.
    let map_2 = btree_2! {
        1 => "tbt", 2 => "TheBestTvarynka"
    };
    println!("{map_2:?}");
}
