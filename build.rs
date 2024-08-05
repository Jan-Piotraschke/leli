// build.rs
fn main() {
    println!("cargo:rustc-link-search=windows");
    println!("cargo:rustc-link-lib=static=sqlite3");
}
