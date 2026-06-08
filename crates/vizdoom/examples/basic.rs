//! Minimal port of the C++ `examples/c++/Basic.cpp` loop.
//!
//! Run with paths pointing at a built engine and scenario, for example:
//!
//! ```sh
//! VIZDOOM_BIN=../../bin cargo run --example basic
//! ```
//!
//! The engine binary and scenario WAD must already be built (see the ViZDoom
//! build instructions); this example only drives the Rust bindings.

use std::env;
use std::path::PathBuf;

use vizdoom::{Button, DoomGame, GameVariable, Mode, ScreenFormat, ScreenResolution};

fn main() -> vizdoom::Result<()> {
    // Directory containing the `vizdoom` engine binary and resource WADs.
    let bin = PathBuf::from(env::var("VIZDOOM_BIN").unwrap_or_else(|_| "../../bin".to_string()));
    let scenarios =
        PathBuf::from(env::var("VIZDOOM_SCENARIOS").unwrap_or_else(|_| "../../scenarios".to_string()));

    let mut game = DoomGame::new()?;
    game.set_vizdoom_path(bin.join("vizdoom"))?;
    game.set_doom_game_path(bin.join("freedoom2.wad"))?;
    game.set_doom_scenario_path(scenarios.join("basic.wad"))?;
    game.set_doom_map("map01")?;

    game.set_screen_resolution(ScreenResolution::Res640x480)?;
    game.set_screen_format(ScreenFormat::Rgb24)?;
    game.set_render_hud(false)?;

    game.add_available_button(Button::MoveLeft)?;
    game.add_available_button(Button::MoveRight)?;
    game.add_available_button(Button::Attack)?;

    game.add_available_game_variable(GameVariable::Ammo2)?;

    game.set_episode_timeout(200)?;
    game.set_episode_start_time(10)?;
    game.set_window_visible(true)?;
    game.set_mode(Mode::Player)?;

    game.init()?;

    let actions = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    let episodes = 10;
    for i in 0..episodes {
        println!("Episode #{}", i + 1);
        game.new_episode()?;

        let mut step = 0usize;
        while !game.is_episode_finished()? {
            if let Some(state) = game.state()? {
                let number = state.number()?;
                let vars = state.game_variables()?;
                let ammo = vars.first().copied().unwrap_or(0.0);

                // Cycle deterministically through the action table.
                let action = &actions[step % actions.len()];
                let reward = game.make_action(action, 1)?;

                println!("State #{number}  ammo2={ammo}  reward={reward}");
            }
            step += 1;
        }

        println!("Episode finished. Total reward: {}", game.total_reward()?);
        println!("************************");
    }

    game.close()?;
    Ok(())
}
