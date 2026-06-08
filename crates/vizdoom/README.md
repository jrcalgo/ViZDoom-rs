# vizdoom

Safe, idiomatic Rust bindings for [ViZDoom](https://github.com/Farama-Foundation/ViZDoom) — a Doom-based platform for reinforcement-learning research that lets agents play **using only visual information** (the screen buffer).

This crate wraps the raw C ABI exposed by [`vizdoom-sys`](../vizdoom-sys) (see `include/ViZDoomC.h`) with RAII handles, typed enums, and `Result`-based error handling. It targets the minimal `DoomGame` surface needed to drive an RL loop: configure the game, start episodes, apply actions, and read observations (screen/auxiliary buffers and game variables).

## Features

- **`DoomGame`** — an RAII handle around the underlying C++ `DoomGame` and the engine process it spawns. Configure it, `init()` it, then drive episodes with `new_episode()` / `make_action()` / `is_episode_finished()`.
- **`GameState`** — an owned snapshot of the game at a single tic, exposing the screen buffer, depth/labels/automap/audio buffers (when enabled), game variables, state number, and tic.
- **Typed enums** mirroring the C++ side exactly (`#[repr(i32)]`, matching discriminants for safe integer-cast across the ABI): [`Mode`], [`Button`], [`GameVariable`], [`ScreenFormat`], [`ScreenResolution`], [`AutomapMode`], [`SamplingRate`].
- **`Error` / `Result`** — a `thiserror`-based error type (`InvalidArgument`, `NotRunning`, `FileNotFound`, `Engine`) built from the C ABI's status codes and thread-local error messages.

## Example

```rust,no_run
use vizdoom::{Button, DoomGame, Mode, ScreenFormat, ScreenResolution};

let mut game = DoomGame::new()?;
game.set_doom_scenario_path("scenarios/basic.wad")?;
game.set_doom_map("map01")?;
game.set_screen_resolution(ScreenResolution::Res320x240)?;
game.set_screen_format(ScreenFormat::Rgb24)?;
game.add_available_button(Button::MoveLeft)?;
game.add_available_button(Button::MoveRight)?;
game.add_available_button(Button::Attack)?;
game.set_mode(Mode::Player)?;
game.init()?;

game.new_episode()?;
while !game.is_episode_finished()? {
    if let Some(state) = game.state()? {
        let _pixels = state.screen_buffer()?;
    }
    let reward = game.make_action(&[1.0, 0.0, 0.0], 1)?;
    let _ = reward;
}
# Ok::<(), vizdoom::Error>(())
```

A fuller, runnable port of the upstream `Basic.cpp` example lives in [`examples/basic.rs`](examples/basic.rs).

## Runtime requirement: the engine binary

`libvizdoom` doesn't run the game itself — at runtime it spawns a separate `vizdoom` engine executable as a subprocess. Before calling [`DoomGame::init`], you must:

1. Build the `vizdoom` engine binary and obtain the IWAD (e.g. `freedoom2.wad`) and scenario WADs, as part of a normal ViZDoom build.
2. Point the game at them via [`DoomGame::set_vizdoom_path`], [`DoomGame::set_doom_game_path`], and [`DoomGame::set_doom_scenario_path`] (or load an equivalent `.cfg` file with [`DoomGame::load_config`]).

Running the bundled example looks like:

```sh
VIZDOOM_BIN=../../bin VIZDOOM_SCENARIOS=../../scenarios cargo run --example basic
```

`VIZDOOM_BIN` should point at the directory containing the `vizdoom` engine binary and `freedoom2.wad`; `VIZDOOM_SCENARIOS` at the directory containing scenario WADs (e.g. `basic.wad`). Both default to `../../bin` and `../../scenarios` respectively.

## Linking `libvizdoom`

This crate depends on [`vizdoom-sys`](../vizdoom-sys), which links the native `libvizdoom` shared library. By default it dynamically links against a prebuilt library — point `VIZDOOM_LIB_DIR` at a directory containing it, or rely on the system linker's default search paths.

Enable the `static-link` feature to instead build `libvizdoom` from the vendored C++ source tree via CMake and link it in directly (only available inside the workspace; see [`vizdoom-sys`](../vizdoom-sys)'s README for details):

```sh
cargo build --features static-link
```

## Concurrency

[`DoomGame`] is `Send` but deliberately **not** `Sync`: every method that touches engine state requires `&mut self` (or relies on state that must not be accessed concurrently). To drive multiple game instances in parallel, give each worker its own `DoomGame`; to share one across threads, wrap it in a `Mutex`.

[`GameState`] borrows buffers owned by the underlying snapshot — the returned slices are tied to the `GameState`'s lifetime and the memory is released on `Drop`. It is also not `Send`/`Sync`.

## Testing

```sh
cargo test -p vizdoom
```

- `tests/contract.rs` checks that the safe enum mirrors stay in sync with the underlying C ABI's integer discriminants.
- `tests/integration.rs` exercises the `DoomGame` / `GameState` wrappers end-to-end (requires a working engine + resources at runtime; see above).

## License

MIT
