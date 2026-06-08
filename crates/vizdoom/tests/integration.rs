//! Integration test exercising the reset/step/observe loop against a real
//! engine.
//!
//! This test is `#[ignore]` by default because it requires a built `vizdoom`
//! engine binary and scenario resources. Provide their locations via the
//! `VIZDOOM_BIN` and `VIZDOOM_SCENARIOS` environment variables and run:
//!
//! ```sh
//! cargo test -p vizdoom -- --ignored
//! ```

use std::env;
use std::path::PathBuf;

use vizdoom::{Button, DoomGame, GameVariable, Mode, ScreenFormat, ScreenResolution};

/// Builds and configures (but does not init) a game pointed at the basic
/// scenario, using `VIZDOOM_BIN` / `VIZDOOM_SCENARIOS` (with repo defaults).
fn configured_game() -> vizdoom::Result<DoomGame> {
    let bin = PathBuf::from(env::var("VIZDOOM_BIN").unwrap_or_else(|_| "../../bin".to_string()));
    let scenarios = PathBuf::from(
        env::var("VIZDOOM_SCENARIOS").unwrap_or_else(|_| "../../scenarios".to_string()),
    );

    let mut game = DoomGame::new()?;
    game.set_vizdoom_path(bin.join("vizdoom"))?;
    game.set_doom_game_path(bin.join("freedoom2.wad"))?;
    game.set_doom_scenario_path(scenarios.join("basic.wad"))?;
    game.set_doom_map("map01")?;
    game.set_screen_resolution(ScreenResolution::Res160x120)?;
    game.set_screen_format(ScreenFormat::Rgb24)?;
    game.set_window_visible(false)?;
    game.set_mode(Mode::Player)?;
    game.add_available_button(Button::MoveLeft)?;
    game.add_available_button(Button::MoveRight)?;
    game.add_available_button(Button::Attack)?;
    game.add_available_game_variable(GameVariable::Ammo2)?;
    game.set_episode_timeout(50)?;
    Ok(game)
}

#[test]
#[ignore = "requires a built vizdoom engine binary and scenario WAD"]
fn reset_step_observe_loop() -> vizdoom::Result<()> {
    let mut game = configured_game()?;

    assert!(game.init()?, "engine failed to start");
    assert_eq!(game.available_buttons_size()?, 3);

    game.new_episode()?;
    assert!(game.is_running()?);

    let expected = game.screen_size()?;
    let mut steps = 0;
    while !game.is_episode_finished()? {
        let state = game.state()?.expect("running episode must have a state");
        let screen = state.screen_buffer()?;
        assert_eq!(screen.len(), expected);
        let _ = game.make_action(&[0.0, 1.0, 0.0], 1)?;
        steps += 1;
    }

    assert!(steps > 0, "episode produced no steps");
    game.close()?;
    Ok(())
}

#[test]
#[ignore = "requires a built vizdoom engine binary and scenario WAD"]
fn state_buffers_and_variables_match_config() -> vizdoom::Result<()> {
    let mut game = configured_game()?;
    assert!(game.init()?);
    game.new_episode()?;

    let expected = game.screen_size()?;
    let state = game.state()?.expect("running episode must have a state");

    // Screen buffer length must equal the reported screen size.
    assert_eq!(state.screen_buffer()?.len(), expected);

    // One registered game variable -> a length-1 variables slice.
    assert_eq!(state.game_variables()?.len(), 1);

    // Disabled buffers report an empty slice (null + 0 across the ABI).
    assert!(state.depth_buffer()?.is_empty());

    game.close()?;
    Ok(())
}

#[test]
#[ignore = "requires a built vizdoom engine binary and scenario WAD"]
fn depth_buffer_is_populated_when_enabled() -> vizdoom::Result<()> {
    let mut game = configured_game()?;
    game.set_depth_buffer_enabled(true)?;
    assert!(game.init()?);
    game.new_episode()?;

    let state = game.state()?.expect("running episode must have a state");
    assert!(!state.depth_buffer()?.is_empty());

    game.close()?;
    Ok(())
}

#[test]
#[ignore = "requires a built vizdoom engine binary and scenario WAD"]
fn total_reward_accumulates_over_episode() -> vizdoom::Result<()> {
    let mut game = configured_game()?;
    assert!(game.init()?);
    game.new_episode()?;

    let mut sum = 0.0;
    while !game.is_episode_finished()? {
        sum += game.make_action(&[0.0, 0.0, 1.0], 1)?;
    }

    // The engine's running total should track the sum of per-action rewards.
    let total = game.total_reward()?;
    assert!((total - sum).abs() < 1e-6, "total {total} vs summed {sum}");

    game.close()?;
    Ok(())
}

#[test]
#[ignore = "requires a built vizdoom engine binary and scenario WAD"]
fn state_snapshots_are_independent() -> vizdoom::Result<()> {
    let mut game = configured_game()?;
    assert!(game.init()?);
    game.new_episode()?;

    // Take a snapshot, advance, take another, then drop the first and confirm
    // the second is still readable (validates the owned state copy).
    let first = game.state()?.expect("state A");
    let first_number = first.number()?;
    let first_len = first.screen_buffer()?.len();

    game.make_action(&[0.0, 1.0, 0.0], 1)?;
    let second = game.state()?.expect("state B");

    drop(first);

    assert_eq!(second.screen_buffer()?.len(), first_len);
    assert!(second.number()? >= first_number);

    game.close()?;
    Ok(())
}
