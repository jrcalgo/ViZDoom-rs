//! Engine-free contract tests for the safe `vizdoom` API.
//!
//! These validate that the safe wrapper maps C ABI statuses onto the right
//! typed errors without spawning the engine.

use vizdoom::{Button, DoomGame, Error, GameVariable};

#[test]
fn new_game_is_not_running() {
    let game = DoomGame::new().expect("new should succeed");
    assert!(!game.is_running().expect("is_running should succeed"));
}

#[test]
fn operations_without_init_return_not_running() {
    let mut game = DoomGame::new().unwrap();

    assert!(matches!(game.state(), Err(Error::NotRunning)));
    assert!(matches!(
        game.get_game_variable(GameVariable::KillCount),
        Err(Error::NotRunning)
    ));
    assert!(matches!(game.last_reward(), Err(Error::NotRunning)));
    assert!(matches!(game.make_action(&[0.0], 1), Err(Error::NotRunning)));
    assert!(matches!(game.respawn_player(), Err(Error::NotRunning)));
    assert!(matches!(game.is_episode_finished(), Err(Error::NotRunning)));
}

#[test]
fn interior_nul_is_invalid_argument() {
    let mut game = DoomGame::new().unwrap();
    assert!(matches!(
        game.set_doom_map("a\0b"),
        Err(Error::InvalidArgument(_))
    ));
}

#[test]
fn missing_config_is_file_not_found() {
    let mut game = DoomGame::new().unwrap();
    assert!(matches!(
        game.load_config("this/path/does/not/exist.cfg"),
        Err(Error::FileNotFound(_))
    ));
}

#[test]
fn button_list_management() {
    let mut game = DoomGame::new().unwrap();

    game.clear_available_buttons().unwrap();
    assert_eq!(game.available_buttons_size().unwrap(), 0);

    game.add_available_button(Button::MoveLeft).unwrap();
    game.add_available_button(Button::MoveRight).unwrap();
    game.add_available_button(Button::Attack).unwrap();
    assert_eq!(game.available_buttons_size().unwrap(), 3);

    game.clear_available_buttons().unwrap();
    assert_eq!(game.available_buttons_size().unwrap(), 0);
}

#[test]
fn engine_free_config_setters_succeed() {
    use vizdoom::{Mode, ScreenFormat, ScreenResolution};

    let mut game = DoomGame::new().unwrap();
    game.set_doom_map("map01").unwrap();
    game.set_mode(Mode::Player).unwrap();
    game.set_seed(42).unwrap();
    game.set_episode_timeout(200).unwrap();
    game.set_screen_resolution(ScreenResolution::Res320x240).unwrap();
    game.set_screen_format(ScreenFormat::Rgb24).unwrap();
    game.set_window_visible(false).unwrap();
    game.set_living_reward(1.0).unwrap();
    game.add_available_game_variable(GameVariable::Ammo2).unwrap();
    game.clear_available_game_variables().unwrap();
}
