use std::env;
use std::path::{Path, PathBuf};

fn main() -> miette::Result<()> {
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

    let src_path = Path::new(&manifest_dir).join("src");

    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&xapian_include_dir, &src_path])
        .extra_clang_args(&["-std=c++17"])
        .build()?;

    // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-deprecated-declarations")
        .include(&src_path)
        .file(Path::new(&manifest_dir).join("src/easy_wrapper.cc"))
        .compile("autocxx-xapian-rs"); // arbitrary library name, pick anything

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

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/easy_wrapper.h");
    println!("cargo:rerun-if-changed=src/easy_wrapper.cc");
    Ok(())
}
