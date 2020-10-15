fn main() {
    // Check if version of rustc is >= 1.34
    if let Some(true) = version_check::is_min_version("1.34.0") {
        println!("cargo:rustc-cfg=try_from");
    }
}
