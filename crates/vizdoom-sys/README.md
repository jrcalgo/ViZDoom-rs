# vizdoom-sys

Raw, unsafe FFI bindings to the [ViZDoom](https://github.com/Farama-Foundation/ViZDoom) C ABI, as declared in `include/ViZDoomC.h`.

This crate is a thin, `#[repr(C)]` mirror of the C ABI: opaque handles, status codes, and `unsafe extern "C"` function declarations. It does not provide ownership, error handling, or type safety beyond what C gives you. **Most users should depend on the [`vizdoom`](../vizdoom) crate instead**, which wraps this one in a safe, idiomatic API.

## What this crate provides

- `VzdGame` / `VzdState` — opaque handle types for a game instance and a state snapshot.
- `VzdStatus` — the status/error code enum returned by fallible C ABI calls (`Ok`, `Error`, `InvalidArgument`, `NotRunning`, `FileNotFound`).
- `unsafe extern "C"` declarations for the full `vzd_game_*` / `vzd_state_*` surface: lifecycle, configuration, action/observation spaces, episode flow, actions/rewards, screen info, and state-buffer accessors.
- `vzd_last_error_message` for retrieving the thread-local error string set by the engine on failure.

See `src/lib.rs` for the full list of declarations — it intentionally mirrors `include/ViZDoomC.h` 1:1.

## Linking `libvizdoom`

This crate links against the native `libvizdoom` shared library. There are two strategies, controlled by the `static-link` feature:

### 1. Dynamic linking (default)

Links against a prebuilt `libvizdoom` at build time:

- Set the `VIZDOOM_LIB_DIR` environment variable to a directory containing the shared library. The build script adds it to the native search path and embeds it as an rpath.
- If unset, the system linker's standard search paths are used (e.g. a system-wide install).

```sh
VIZDOOM_LIB_DIR=/path/to/lib cargo build
```

### 2. Static build from source (`static-link` feature)

Enabling the `static-link` feature drives the project's CMake build from the vendored C++ source tree (`CMakeLists.txt`, `include/`, `src/`, `cmake_modules/`, ...) and links the resulting `libvizdoom_shared` target directly:

```sh
cargo build --features static-link
```

This requires the full ViZDoom source tree to be present at the repository root, two levels above this crate (`<repo>/crates/vizdoom-sys` → `<repo>`). It therefore **only works when building inside the workspace**, not when depending on a published crates.io package. It also pulls in the optional `cmake` build-dependency.

Both strategies additionally link the platform C++ standard library (`stdc++` on Linux/most Unix, `c++` on macOS/iOS, and the MSVC runtime implicitly on Windows).

## Runtime requirement: the engine binary

`libvizdoom` does not run the game itself — at runtime it spawns a separate `vizdoom` engine executable as a subprocess. Regardless of which linking strategy you use:

- That engine binary, the IWAD (e.g. `freedoom2.wad`), and any scenario WADs must be built/obtained separately (see the upstream ViZDoom build instructions).
- Their paths must be supplied to the game instance — via `vzd_game_set_vizdoom_path` / `vzd_game_set_doom_game_path` / `vzd_game_set_doom_scenario_path`, or through a `.cfg` config file — *before* calling `vzd_game_init`.

## Safety

Every function in this crate is `unsafe`: it operates on raw pointers to opaque C++ objects and trusts the caller to uphold the C ABI's invariants (valid, non-aliased handles; correctly-sized output buffers; NUL-terminated C strings; etc.). No lifetime, ownership, or thread-safety guarantees are enforced by the Rust type system here — those are the responsibility of the [`vizdoom`](../vizdoom) crate's safe wrappers.

## Testing

```sh
cargo test -p vizdoom-sys
```

`tests/raw_ffi.rs` exercises the raw C ABI surface directly (smoke-testing handle lifecycle, status codes, and buffer accessors) without the safe wrapper layer.

## License

MIT
