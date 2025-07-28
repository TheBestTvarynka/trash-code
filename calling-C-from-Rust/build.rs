use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=static=simplelib"); // or "static=simplelib"
    println!("cargo:rustc-link-search=native=./");

    let bindings = bindgen::Builder::default()
        .header("simplelib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = Path::new("./src/").join("simplelib.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
