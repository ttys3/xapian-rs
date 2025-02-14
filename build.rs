use cxx_build::CFG;
use std::env;
use std::path::{Path, PathBuf};

fn main() -> miette::Result<()> {
    if cfg!(trybuild) {
        return Ok(());
    }
    let xapian_15 = env::var("CARGO_FEATURE_XAPIAN_1_5").is_ok();
    let pkg_config_lib_name = if xapian_15 { "xapian-core-1.5" } else { "xapian-core" };
    let link_lib_name = if xapian_15 { "xapian-1.5" } else { "xapian" };

    let mut vendored_xapian = env::var("CARGO_FEATURE_VENDORED_XAPIAN").is_ok();
    let try_to_use_system_xapian = !vendored_xapian;

    // sudo dnf install -y xapian-core-devel xapian-core-libs
    if try_to_use_system_xapian {
        let mut cfg = pkg_config::Config::new();
        // /usr/lib64/pkgconfig/xapian-core.pc
        // pkg-config --modversion xapian-core
        if let Ok(lib) = cfg.range_version("1.4.0".."1.5.99").probe(pkg_config_lib_name) {
            for include in &lib.include_paths {
                println!("cargo:root={}", include.display());
            }
            println!(
                "cargo:warning=found and use system xapian: {}, include_paths: {:?}, libs: {:?}",
                pkg_config_lib_name, &lib.include_paths, &lib.libs
            );
        } else {
            println!("cargo:warning=failed to find system xapian, falling back to vendored");
            vendored_xapian = true;
        }
    } else {
        println!("cargo:warning=use vendored xapian");
    }

    // include path
    // xapian 1.5 /usr/include/xapian-1.5/xapian.h and /usr/include/xapian-1.5/xapian/*.h
    // xapian 1.4 /usr/include/xapian.h and /usr/include/xapian/*.h
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let sys_include_dir = if xapian_15 { "/usr/include/xapian-1.5" } else { "/usr/include" };

    let xapian_include_dir = if vendored_xapian {
        if xapian_15 {
            Path::new(&manifest_dir).join("include/xapian-1.5")
        } else {
            Path::new(&manifest_dir).join("include/xapian-1.4")
        }
    } else {
        PathBuf::from(sys_include_dir)
    };
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
    // static, dylib, framework, link-arg
    if vendored_xapian {
        println!("cargo:rustc-link-search=all=xapian/xapian-core/.libs");
        println!("cargo:rustc-link-lib=static={}", link_lib_name);
        println!("cargo:rustc-link-lib=dylib={}", link_lib_name);
        // export LD_LIBRARY_PATH=xapian/xapian-core/.libs
    } else {
        println!("cargo:rustc-link-lib={}", link_lib_name);
    }
    println!("cargo:warning=link lib name: {}", link_lib_name);

    println!("cargo:rerun-if-changed=xapian-bind.cc");
    println!("cargo:rerun-if-changed=xapian-bind.h");
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
