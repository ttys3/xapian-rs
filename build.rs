use cxx_build::CFG;
use std::env;
use std::path::Path;

fn main() {
    if cfg!(trybuild) {
        return;
    }

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let xapian_include_dir = Path::new(&manifest_dir).join("include");
    CFG.exported_header_dirs.push(&xapian_include_dir);

    // https://docs.rs/cc/1.0.79/cc/struct.Build.html#method.compile
    // The output string argument determines the file name for the compiled library.
    // The Rust compiler will create an assembly named “lib”+output+“.a”. MSVC will create a file named output+“.lib”.

    // https://lists.xapian.org/pipermail/xapian-discuss/2023-March/009961.html
    // Currently master requires C++17 to build xapian
    let sources = vec!["src/lib.rs"];
    cxx_build::bridges(sources)
        .file("xapian-bind.cc")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-deprecated-declarations")
        .compile("xapian-rs");

    // external lib
    println!("cargo:rustc-link-lib=xapian");
    println!("cargo:rustc-link-lib=m");

    println!("cargo:rerun-if-changed=xapian-bind.cc");
    println!("cargo:rerun-if-changed=xapian-bind.h");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
