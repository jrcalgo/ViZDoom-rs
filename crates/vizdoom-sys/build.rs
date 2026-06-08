//! Build script for `vizdoom-sys`.
//!
//! Two strategies (see the `prebuilt` feature and `VIZDOOM_LIB_DIR`):
//!
//! 1. Default: drive the project's CMake build via the `cmake` crate, building
//!    the shared `libvizdoom` target (which bundles its Boost/threads deps),
//!    then link against it.
//! 2. Prebuilt override: if `VIZDOOM_LIB_DIR` is set (or the `prebuilt` feature
//!    is enabled), skip the CMake build and link a prebuilt library from that
//!    directory.
//!
//! Note: `libvizdoom` spawns a separate `vizdoom` engine executable at runtime.
//! That binary (and the scenario/IWAD resources) must be built separately and
//! its path supplied through `set_vizdoom_path` / a config file.

use std::env;
use std::path::{Path, PathBuf};

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

fn use_prebuilt(lib_dir: &str) {
    let path = PathBuf::from(lib_dir);
    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", path.display());
    println!("cargo:rustc-link-lib=dylib=vizdoom");
    link_cpp_stdlib();
}

fn build_with_cmake(root: &Path) {
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
    let root = repo_root();

    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}",
        root.join("include/ViZDoomC.h").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        root.join("src/lib/ViZDoomC.cpp").display()
    );
    println!("cargo:rerun-if-env-changed=VIZDOOM_LIB_DIR");

    let prebuilt_feature = env::var("CARGO_FEATURE_PREBUILT").is_ok();
    match env::var("VIZDOOM_LIB_DIR") {
        Ok(dir) if !dir.is_empty() => use_prebuilt(&dir),
        _ if prebuilt_feature => panic!(
            "the `prebuilt` feature is enabled but VIZDOOM_LIB_DIR is not set; \
             point it at a directory containing libvizdoom"
        ),
        _ => build_with_cmake(&root),
    }
}
