//! Build script for `vizdoom-sys`.
//!
//! Two strategies (see the `static-link` feature and `VIZDOOM_LIB_DIR`):
//!
//! 1. Default: dynamic link against a prebuilt `libvizdoom`. Set
//!    `VIZDOOM_LIB_DIR` to point at a directory containing it; otherwise the
//!    system linker's standard search paths are used.
//! 2. Static: enable the `static-link` feature to drive the project's CMake
//!    build from its vendored C++ source tree (building the shared
//!    `libvizdoom` target, which bundles its Boost/threads deps) and link
//!    against the result. This requires the full ViZDoom source tree, which
//!    is only available when building inside the workspace (not from a
//!    published crates.io package).
//!
//! Note: `libvizdoom` spawns a separate `vizdoom` engine executable at runtime.
//! That binary (and the scenario/IWAD resources) must be built separately and
//! its path supplied through `set_vizdoom_path` / a config file.

use std::env;
#[cfg(feature = "static-link")]
use std::path::Path;
use std::path::PathBuf;

#[cfg(feature = "static-link")]
fn repo_root() -> PathBuf {
    // crates/vizdoom-sys -> repo root is two levels up.
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("vizdoom-sys must live under <repo>/crates/vizdoom-sys")
        .to_path_buf()
}

fn target_os() -> String {
    env::var("CARGO_CFG_TARGET_OS").unwrap_or_default()
}

/// Emit the C++ standard library link directive for the target platform.
fn link_cpp_stdlib() {
    match target_os().as_str() {
        "macos" | "ios" => println!("cargo:rustc-link-lib=dylib=c++"),
        "windows" => { /* MSVC links the C++ runtime automatically. */ }
        _ => println!("cargo:rustc-link-lib=dylib=stdc++"),
    }
}

/// Dynamic link against a (possibly prebuilt) `libvizdoom`. If
/// `VIZDOOM_LIB_DIR` is set, search there and embed an rpath; otherwise rely
/// on the linker's default search paths (e.g. a system install).
#[cfg(not(feature = "static-link"))]
fn link_dynamic() {
    if let Ok(dir) = env::var("VIZDOOM_LIB_DIR") {
        if !dir.is_empty() {
            let path = PathBuf::from(&dir);
            println!("cargo:rustc-link-search=native={}", path.display());
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", path.display());
        }
    }
    println!("cargo:rustc-link-lib=dylib=vizdoom");
    link_cpp_stdlib();
}

/// Build `libvizdoom` from the vendored C++ source tree via CMake and link it
/// in. Only available inside the workspace, where the source tree exists.
#[cfg(feature = "static-link")]
fn link_static_from_source(root: &Path) {
    println!(
        "cargo:rerun-if-changed={}",
        root.join("include/ViZDoomC.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        root.join("src/lib/ViZDoomC.cpp").display()
    );

    // Build the shared core library target. It links its Boost/threads deps
    // privately, so we only need to link `vizdoom` itself.
    let dst = cmake::Config::new(root)
        .define("BUILD_PYTHON", "OFF")
        .define("BUILD_ENGINE", "ON")
        .build_target("libvizdoom_shared")
        .build();

    // CMake places the artifact in <binary_dir>/bin (VIZDOOM_OUTPUT_DIR).
    let lib_dir = dst.join("build").join("bin");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=vizdoom");
    link_cpp_stdlib();

    // Surface the build dir so dependents/tests can locate the engine binary.
    println!("cargo:lib_dir={}", lib_dir.display());
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=VIZDOOM_LIB_DIR");

    #[cfg(feature = "static-link")]
    link_static_from_source(&repo_root());

    #[cfg(not(feature = "static-link"))]
    link_dynamic();
}
