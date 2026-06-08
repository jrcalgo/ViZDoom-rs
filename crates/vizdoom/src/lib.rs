//! Safe, idiomatic Rust bindings for [ViZDoom](https://github.com/Farama-Foundation/ViZDoom).
//!
//! This crate wraps the C ABI exposed by `vizdoom-sys` (see
//! `include/ViZDoomC.h`) with RAII handles, typed enums, and `Result`-based
//! error handling. It targets the minimal `DoomGame` surface needed to drive a
//! reinforcement-learning loop: configure the game, start episodes, apply
//! actions, and read observations (screen/auxiliary buffers and game
//! variables).
//!
//! # Runtime requirement
//!
//! `libvizdoom` launches a separate `vizdoom` engine executable. Build that
//! binary (and the scenario/IWAD resources) as part of the normal ViZDoom build
//! and point the game at it via [`DoomGame::set_vizdoom_path`] (or a config
//! file) before calling [`DoomGame::init`].
//!
//! # Example
//!
//! ```no_run
//! use vizdoom::{Button, DoomGame, Mode, ScreenFormat, ScreenResolution};
//!
//! # fn main() -> vizdoom::Result<()> {
//! let mut game = DoomGame::new()?;
//! game.set_doom_scenario_path("scenarios/basic.wad")?;
//! game.set_doom_map("map01")?;
//! game.set_screen_resolution(ScreenResolution::Res320x240)?;
//! game.set_screen_format(ScreenFormat::Rgb24)?;
//! game.add_available_button(Button::MoveLeft)?;
//! game.add_available_button(Button::MoveRight)?;
//! game.add_available_button(Button::Attack)?;
//! game.set_mode(Mode::Player)?;
//! game.init()?;
//!
//! game.new_episode()?;
//! while !game.is_episode_finished()? {
//!     if let Some(state) = game.state()? {
//!         let _pixels = state.screen_buffer()?;
//!     }
//!     let reward = game.make_action(&[1.0, 0.0, 0.0], 1)?;
//!     let _ = reward;
//! }
//! # Ok(())
//! # }
//! ```

mod enums;
mod error;
mod game;
mod state;

pub use enums::{
    AutomapMode, Button, GameVariable, Mode, SamplingRate, ScreenFormat, ScreenResolution,
};
pub use error::{Error, Result};
pub use game::DoomGame;
pub use state::GameState;
