//! Contract tests for the raw C ABI symbols exposed by `vizdoom-sys`.
//!
//! These mirror the C++ `test_c_abi_contract.cpp` tier at the lowest Rust layer
//! and run without spawning the engine. They validate the FFI boundary: status
//! mapping, null-handle guards, argument ordering, and thread-local error
//! messages.

use std::ffi::{CStr, CString};
use std::os::raw::{c_double, c_int};
use std::ptr;

use vizdoom_sys as sys;

/// Constructs a fresh game handle, asserting success.
unsafe fn new_game() -> *mut sys::VzdGame {
    let mut game: *mut sys::VzdGame = ptr::null_mut();
    assert_eq!(sys::vzd_game_new(&mut game), sys::VzdStatus::Ok);
    assert!(!game.is_null());
    game
}

#[test]
fn new_and_free() {
    unsafe {
        let game = new_game();

        let mut running: c_int = -1;
        assert_eq!(
            sys::vzd_game_is_running(game, &mut running),
            sys::VzdStatus::Ok
        );
        assert_eq!(running, 0);

        sys::vzd_game_free(game);

        // Null out-param is rejected; null free is a no-op.
        assert_eq!(sys::vzd_game_new(ptr::null_mut()), sys::VzdStatus::InvalidArgument);
        sys::vzd_game_free(ptr::null_mut());
    }
}

#[test]
fn null_handles_are_rejected() {
    unsafe {
        let mut out: c_int = 0;
        let mut sout: usize = 0;
        let mut data: *const u8 = ptr::null();
        let mut len: usize = 0;

        assert_eq!(sys::vzd_game_init(ptr::null_mut(), &mut out), sys::VzdStatus::InvalidArgument);
        assert_eq!(sys::vzd_game_close(ptr::null_mut()), sys::VzdStatus::InvalidArgument);
        assert_eq!(
            sys::vzd_game_is_running(ptr::null_mut(), &mut out),
            sys::VzdStatus::InvalidArgument
        );
        assert_eq!(
            sys::vzd_game_get_available_buttons_size(ptr::null_mut(), &mut sout),
            sys::VzdStatus::InvalidArgument
        );
        assert_eq!(
            sys::vzd_game_get_state(ptr::null_mut(), ptr::null_mut()),
            sys::VzdStatus::InvalidArgument
        );

        let mut u32_out: u32 = 0;
        assert_eq!(
            sys::vzd_state_number(ptr::null(), &mut u32_out),
            sys::VzdStatus::InvalidArgument
        );
        assert_eq!(
            sys::vzd_state_screen_buffer(ptr::null(), &mut data, &mut len),
            sys::VzdStatus::InvalidArgument
        );
        sys::vzd_state_free(ptr::null_mut());
    }
}

#[test]
fn null_out_params_are_rejected() {
    unsafe {
        let game = new_game();
        assert_eq!(
            sys::vzd_game_is_running(game, ptr::null_mut()),
            sys::VzdStatus::InvalidArgument
        );
        assert_eq!(
            sys::vzd_game_get_available_buttons_size(game, ptr::null_mut()),
            sys::VzdStatus::InvalidArgument
        );
        assert_eq!(
            sys::vzd_game_get_state(game, ptr::null_mut()),
            sys::VzdStatus::InvalidArgument
        );
        sys::vzd_game_free(game);
    }
}

#[test]
fn operations_on_non_running_game_map_to_not_running() {
    unsafe {
        let game = new_game();

        let mut out: c_int = 0;
        let mut reward: c_double = 0.0;
        let mut var: c_double = 0.0;
        let mut state: *mut sys::VzdState = ptr::null_mut();
        let action = [0.0f64, 0.0, 0.0];

        assert_eq!(sys::vzd_game_get_state(game, &mut state), sys::VzdStatus::NotRunning);
        assert_eq!(
            sys::vzd_game_make_action(game, action.as_ptr(), action.len(), 1, &mut reward),
            sys::VzdStatus::NotRunning
        );
        assert_eq!(
            sys::vzd_game_set_action(game, action.as_ptr(), action.len()),
            sys::VzdStatus::NotRunning
        );
        assert_eq!(
            sys::vzd_game_advance_action(game, 1, 1),
            sys::VzdStatus::NotRunning
        );
        assert_eq!(
            sys::vzd_game_get_game_variable(game, 0, &mut var),
            sys::VzdStatus::NotRunning
        );
        assert_eq!(
            sys::vzd_game_is_episode_finished(game, &mut out),
            sys::VzdStatus::NotRunning
        );
        assert_eq!(sys::vzd_game_respawn_player(game), sys::VzdStatus::NotRunning);

        sys::vzd_game_free(game);
    }
}

#[test]
fn set_action_argument_ordering() {
    unsafe {
        let game = new_game();
        // Null data + non-zero length is rejected before reaching the engine.
        assert_eq!(
            sys::vzd_game_set_action(game, ptr::null(), 5),
            sys::VzdStatus::InvalidArgument
        );
        // Null data + zero length passes the guard, then hits not-running.
        assert_eq!(
            sys::vzd_game_set_action(game, ptr::null(), 0),
            sys::VzdStatus::NotRunning
        );
        sys::vzd_game_free(game);
    }
}

#[test]
fn engine_free_config_and_button_list() {
    unsafe {
        let game = new_game();

        let map = CString::new("map01").unwrap();
        assert_eq!(sys::vzd_game_set_doom_map(game, map.as_ptr()), sys::VzdStatus::Ok);
        assert_eq!(sys::vzd_game_set_mode(game, 0), sys::VzdStatus::Ok);
        assert_eq!(sys::vzd_game_set_seed(game, 42), sys::VzdStatus::Ok);
        assert_eq!(sys::vzd_game_set_episode_timeout(game, 200), sys::VzdStatus::Ok);

        let mut size: usize = 99;
        assert_eq!(sys::vzd_game_clear_available_buttons(game), sys::VzdStatus::Ok);
        assert_eq!(
            sys::vzd_game_get_available_buttons_size(game, &mut size),
            sys::VzdStatus::Ok
        );
        assert_eq!(size, 0);

        // MOVE_LEFT=11, MOVE_RIGHT=10, ATTACK=0
        assert_eq!(sys::vzd_game_add_available_button(game, 11, -1.0), sys::VzdStatus::Ok);
        assert_eq!(sys::vzd_game_add_available_button(game, 10, -1.0), sys::VzdStatus::Ok);
        assert_eq!(sys::vzd_game_add_available_button(game, 0, -1.0), sys::VzdStatus::Ok);
        assert_eq!(
            sys::vzd_game_get_available_buttons_size(game, &mut size),
            sys::VzdStatus::Ok
        );
        assert_eq!(size, 3);

        assert_eq!(sys::vzd_game_clear_available_buttons(game), sys::VzdStatus::Ok);
        assert_eq!(
            sys::vzd_game_get_available_buttons_size(game, &mut size),
            sys::VzdStatus::Ok
        );
        assert_eq!(size, 0);

        sys::vzd_game_free(game);
    }
}

#[test]
fn missing_config_maps_to_file_not_found() {
    unsafe {
        let game = new_game();
        let path = CString::new("this/path/does/not/exist.cfg").unwrap();
        let mut success: c_int = -1;
        assert_eq!(
            sys::vzd_game_load_config(game, path.as_ptr(), &mut success),
            sys::VzdStatus::FileNotFound
        );
        sys::vzd_game_free(game);
    }
}

#[test]
fn error_message_is_set_then_cleared() {
    unsafe {
        let game = new_game();

        let action = [0.0f64];
        let mut reward: c_double = 0.0;
        assert_eq!(
            sys::vzd_game_make_action(game, action.as_ptr(), action.len(), 1, &mut reward),
            sys::VzdStatus::NotRunning
        );
        let msg = CStr::from_ptr(sys::vzd_last_error_message());
        assert!(!msg.to_bytes().is_empty());

        // A subsequent success clears the thread-local message.
        assert_eq!(sys::vzd_game_set_mode(game, 0), sys::VzdStatus::Ok);
        let cleared = CStr::from_ptr(sys::vzd_last_error_message());
        assert!(cleared.to_bytes().is_empty());

        sys::vzd_game_free(game);
    }
}
