fn main() {
    println!("cargo:rustc-link-lib=dylib=simplelib"); // or "static=simplelib"
    println!("cargo:rustc-link-search=native=./");
}
