//! Raw, unsafe FFI declarations for the ViZDoom C ABI.
//!
//! These mirror `include/ViZDoomC.h` exactly. Prefer the safe `vizdoom` crate;
//! this crate exists only to expose the linked symbols.

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_double, c_int};

/// Opaque handle to a ViZDoom game instance.
#[repr(C)]
pub struct VzdGame {
    _private: [u8; 0],
}

/// Opaque handle to a snapshot of a game state.
#[repr(C)]
pub struct VzdState {
    _private: [u8; 0],
}

/// Status codes returned by fallible C ABI functions. Mirrors `VzdStatus`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VzdStatus {
    Ok = 0,
    Error = 1,
    InvalidArgument = 2,
    NotRunning = 3,
    FileNotFound = 4,
}

unsafe extern "C" {
    pub fn vzd_last_error_message() -> *const c_char;

    // Lifecycle
    pub fn vzd_game_new(out_game: *mut *mut VzdGame) -> VzdStatus;
    pub fn vzd_game_free(game: *mut VzdGame);
    pub fn vzd_game_init(game: *mut VzdGame, out_success: *mut c_int) -> VzdStatus;
    pub fn vzd_game_close(game: *mut VzdGame) -> VzdStatus;
    pub fn vzd_game_is_running(game: *mut VzdGame, out_running: *mut c_int) -> VzdStatus;

    // Configuration
    pub fn vzd_game_load_config(
        game: *mut VzdGame,
        path: *const c_char,
        out_success: *mut c_int,
    ) -> VzdStatus;
    pub fn vzd_game_set_config(
        game: *mut VzdGame,
        config: *const c_char,
        out_success: *mut c_int,
    ) -> VzdStatus;
    pub fn vzd_game_set_vizdoom_path(game: *mut VzdGame, path: *const c_char) -> VzdStatus;
    pub fn vzd_game_set_doom_game_path(game: *mut VzdGame, path: *const c_char) -> VzdStatus;
    pub fn vzd_game_set_doom_scenario_path(game: *mut VzdGame, path: *const c_char) -> VzdStatus;
    pub fn vzd_game_set_doom_map(game: *mut VzdGame, map: *const c_char) -> VzdStatus;
    pub fn vzd_game_set_mode(game: *mut VzdGame, mode: c_int) -> VzdStatus;
    pub fn vzd_game_set_seed(game: *mut VzdGame, seed: u32) -> VzdStatus;
    pub fn vzd_game_set_episode_timeout(game: *mut VzdGame, tics: u32) -> VzdStatus;
    pub fn vzd_game_set_episode_start_time(game: *mut VzdGame, tics: u32) -> VzdStatus;
    pub fn vzd_game_set_screen_resolution(game: *mut VzdGame, resolution: c_int) -> VzdStatus;
    pub fn vzd_game_set_screen_format(game: *mut VzdGame, format: c_int) -> VzdStatus;
    pub fn vzd_game_set_window_visible(game: *mut VzdGame, visible: c_int) -> VzdStatus;
    pub fn vzd_game_set_render_hud(game: *mut VzdGame, hud: c_int) -> VzdStatus;
    pub fn vzd_game_set_living_reward(game: *mut VzdGame, reward: c_double) -> VzdStatus;
    pub fn vzd_game_set_death_penalty(game: *mut VzdGame, penalty: c_double) -> VzdStatus;
    pub fn vzd_game_set_depth_buffer_enabled(game: *mut VzdGame, enabled: c_int) -> VzdStatus;
    pub fn vzd_game_set_labels_buffer_enabled(game: *mut VzdGame, enabled: c_int) -> VzdStatus;
    pub fn vzd_game_set_automap_buffer_enabled(game: *mut VzdGame, enabled: c_int) -> VzdStatus;
    pub fn vzd_game_set_audio_buffer_enabled(game: *mut VzdGame, enabled: c_int) -> VzdStatus;

    // Action / observation spaces
    pub fn vzd_game_add_available_button(
        game: *mut VzdGame,
        button: c_int,
        max_value: c_double,
    ) -> VzdStatus;
    pub fn vzd_game_clear_available_buttons(game: *mut VzdGame) -> VzdStatus;
    pub fn vzd_game_get_available_buttons_size(
        game: *mut VzdGame,
        out_size: *mut usize,
    ) -> VzdStatus;
    pub fn vzd_game_add_available_game_variable(game: *mut VzdGame, variable: c_int) -> VzdStatus;
    pub fn vzd_game_clear_available_game_variables(game: *mut VzdGame) -> VzdStatus;
    pub fn vzd_game_get_game_variable(
        game: *mut VzdGame,
        variable: c_int,
        out_value: *mut c_double,
    ) -> VzdStatus;

    // Episode flow
    pub fn vzd_game_new_episode(game: *mut VzdGame, recording_path: *const c_char) -> VzdStatus;
    pub fn vzd_game_is_new_episode(game: *mut VzdGame, out_value: *mut c_int) -> VzdStatus;
    pub fn vzd_game_is_episode_finished(game: *mut VzdGame, out_value: *mut c_int) -> VzdStatus;
    pub fn vzd_game_is_episode_timeout_reached(
        game: *mut VzdGame,
        out_value: *mut c_int,
    ) -> VzdStatus;
    pub fn vzd_game_is_player_dead(game: *mut VzdGame, out_value: *mut c_int) -> VzdStatus;
    pub fn vzd_game_respawn_player(game: *mut VzdGame) -> VzdStatus;

    // Actions / rewards
    pub fn vzd_game_set_action(
        game: *mut VzdGame,
        actions: *const c_double,
        actions_len: usize,
    ) -> VzdStatus;
    pub fn vzd_game_advance_action(
        game: *mut VzdGame,
        tics: u32,
        update_state: c_int,
    ) -> VzdStatus;
    pub fn vzd_game_make_action(
        game: *mut VzdGame,
        actions: *const c_double,
        actions_len: usize,
        tics: u32,
        out_reward: *mut c_double,
    ) -> VzdStatus;
    pub fn vzd_game_get_last_reward(game: *mut VzdGame, out_reward: *mut c_double) -> VzdStatus;
    pub fn vzd_game_get_total_reward(game: *mut VzdGame, out_reward: *mut c_double) -> VzdStatus;

    // Screen info
    pub fn vzd_game_get_screen_width(game: *mut VzdGame, out_value: *mut c_int) -> VzdStatus;
    pub fn vzd_game_get_screen_height(game: *mut VzdGame, out_value: *mut c_int) -> VzdStatus;
    pub fn vzd_game_get_screen_channels(game: *mut VzdGame, out_value: *mut c_int) -> VzdStatus;
    pub fn vzd_game_get_screen_size(game: *mut VzdGame, out_value: *mut usize) -> VzdStatus;
    pub fn vzd_game_get_screen_format(game: *mut VzdGame, out_value: *mut c_int) -> VzdStatus;

    // State snapshot
    pub fn vzd_game_get_state(game: *mut VzdGame, out_state: *mut *mut VzdState) -> VzdStatus;
    pub fn vzd_state_free(state: *mut VzdState);
    pub fn vzd_state_number(state: *const VzdState, out_value: *mut u32) -> VzdStatus;
    pub fn vzd_state_tic(state: *const VzdState, out_value: *mut u32) -> VzdStatus;
    pub fn vzd_state_game_variables(
        state: *const VzdState,
        out_data: *mut *const c_double,
        out_len: *mut usize,
    ) -> VzdStatus;
    pub fn vzd_state_screen_buffer(
        state: *const VzdState,
        out_data: *mut *const u8,
        out_len: *mut usize,
    ) -> VzdStatus;
    pub fn vzd_state_depth_buffer(
        state: *const VzdState,
        out_data: *mut *const u8,
        out_len: *mut usize,
    ) -> VzdStatus;
    pub fn vzd_state_labels_buffer(
        state: *const VzdState,
        out_data: *mut *const u8,
        out_len: *mut usize,
    ) -> VzdStatus;
    pub fn vzd_state_automap_buffer(
        state: *const VzdState,
        out_data: *mut *const u8,
        out_len: *mut usize,
    ) -> VzdStatus;
    pub fn vzd_state_audio_buffer(
        state: *const VzdState,
        out_data: *mut *const i16,
        out_len: *mut usize,
    ) -> VzdStatus;
}
