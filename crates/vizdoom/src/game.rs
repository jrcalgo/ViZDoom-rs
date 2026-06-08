//! Safe wrapper around `vizdoom::DoomGame` via the C ABI.

use std::ffi::CString;
use std::os::raw::c_int;
use std::path::Path;
use std::ptr::NonNull;

use vizdoom_sys as sys;

use crate::enums::{Button, GameVariable, Mode, ScreenFormat, ScreenResolution};
use crate::error::{check, Error, Result};
use crate::state::GameState;

/// A ViZDoom game instance.
///
/// This owns the underlying C++ `DoomGame` and the engine process it spawns on
/// [`init`](DoomGame::init). It is `Send` (it may be moved between threads) but
/// deliberately not `Sync`: all methods take `&mut self` or rely on engine
/// state that must not be touched concurrently. To use one game across worker
/// threads, place it behind a `Mutex` or give each worker its own instance.
pub struct DoomGame {
    raw: NonNull<sys::VzdGame>,
}

// SAFETY: ownership of the handle is exclusive and methods never alias it
// across threads simultaneously. Not `Sync`: concurrent access is unsound.
unsafe impl Send for DoomGame {}

impl DoomGame {
    /// Creates a new, uninitialized game. Configure it, then call
    /// [`init`](DoomGame::init).
    pub fn new() -> Result<Self> {
        let mut raw: *mut sys::VzdGame = std::ptr::null_mut();
        check(unsafe { sys::vzd_game_new(&mut raw) })?;
        let raw = NonNull::new(raw)
            .ok_or_else(|| Error::Engine("vzd_game_new returned null".to_string()))?;
        Ok(DoomGame { raw })
    }

    /// Initializes the game, spawning the engine. Returns whether it started.
    pub fn init(&mut self) -> Result<bool> {
        let mut success: c_int = 0;
        check(unsafe { sys::vzd_game_init(self.raw.as_ptr(), &mut success) })?;
        Ok(success != 0)
    }

    /// Closes the game and the engine process. The game can be reconfigured and
    /// re-initialized afterwards.
    pub fn close(&mut self) -> Result<()> {
        check(unsafe { sys::vzd_game_close(self.raw.as_ptr()) })
    }

    /// Whether the engine is currently running.
    pub fn is_running(&self) -> Result<bool> {
        self.get_bool(sys::vzd_game_is_running)
    }

    // ---- Configuration ----------------------------------------------------

    /// Loads a `.cfg` configuration file. Returns whether all keys were valid.
    pub fn load_config(&mut self, path: impl AsRef<Path>) -> Result<bool> {
        let c = path_to_cstring(path.as_ref())?;
        let mut success: c_int = 0;
        check(unsafe { sys::vzd_game_load_config(self.raw.as_ptr(), c.as_ptr(), &mut success) })?;
        Ok(success != 0)
    }

    /// Applies configuration from a string (same syntax as a `.cfg` file).
    pub fn set_config(&mut self, config: &str) -> Result<bool> {
        let c = to_cstring(config)?;
        let mut success: c_int = 0;
        check(unsafe { sys::vzd_game_set_config(self.raw.as_ptr(), c.as_ptr(), &mut success) })?;
        Ok(success != 0)
    }

    /// Sets the path to the `vizdoom` engine executable.
    pub fn set_vizdoom_path(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.set_path(sys::vzd_game_set_vizdoom_path, path.as_ref())
    }

    /// Sets the path to the Doom IWAD game resource.
    pub fn set_doom_game_path(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.set_path(sys::vzd_game_set_doom_game_path, path.as_ref())
    }

    /// Sets the path to the scenario WAD.
    pub fn set_doom_scenario_path(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.set_path(sys::vzd_game_set_doom_scenario_path, path.as_ref())
    }

    /// Sets the starting map (e.g. `"map01"`).
    pub fn set_doom_map(&mut self, map: &str) -> Result<()> {
        let c = to_cstring(map)?;
        check(unsafe { sys::vzd_game_set_doom_map(self.raw.as_ptr(), c.as_ptr()) })
    }

    /// Sets the engine [`Mode`].
    pub fn set_mode(&mut self, mode: Mode) -> Result<()> {
        check(unsafe { sys::vzd_game_set_mode(self.raw.as_ptr(), mode as c_int) })
    }

    /// Sets the RNG seed for determinism.
    pub fn set_seed(&mut self, seed: u32) -> Result<()> {
        check(unsafe { sys::vzd_game_set_seed(self.raw.as_ptr(), seed) })
    }

    /// Sets the episode timeout in tics.
    pub fn set_episode_timeout(&mut self, tics: u32) -> Result<()> {
        check(unsafe { sys::vzd_game_set_episode_timeout(self.raw.as_ptr(), tics) })
    }

    /// Sets the number of tics skipped at the start of each episode.
    pub fn set_episode_start_time(&mut self, tics: u32) -> Result<()> {
        check(unsafe { sys::vzd_game_set_episode_start_time(self.raw.as_ptr(), tics) })
    }

    /// Sets the screen resolution.
    pub fn set_screen_resolution(&mut self, resolution: ScreenResolution) -> Result<()> {
        check(unsafe {
            sys::vzd_game_set_screen_resolution(self.raw.as_ptr(), resolution as c_int)
        })
    }

    /// Sets the screen pixel format.
    pub fn set_screen_format(&mut self, format: ScreenFormat) -> Result<()> {
        check(unsafe { sys::vzd_game_set_screen_format(self.raw.as_ptr(), format as c_int) })
    }

    /// Shows or hides the engine window.
    pub fn set_window_visible(&mut self, visible: bool) -> Result<()> {
        self.set_bool(sys::vzd_game_set_window_visible, visible)
    }

    /// Enables or disables HUD rendering.
    pub fn set_render_hud(&mut self, hud: bool) -> Result<()> {
        self.set_bool(sys::vzd_game_set_render_hud, hud)
    }

    /// Sets the per-tic living reward.
    pub fn set_living_reward(&mut self, reward: f64) -> Result<()> {
        check(unsafe { sys::vzd_game_set_living_reward(self.raw.as_ptr(), reward) })
    }

    /// Sets the death penalty.
    pub fn set_death_penalty(&mut self, penalty: f64) -> Result<()> {
        check(unsafe { sys::vzd_game_set_death_penalty(self.raw.as_ptr(), penalty) })
    }

    /// Enables or disables the depth buffer.
    pub fn set_depth_buffer_enabled(&mut self, enabled: bool) -> Result<()> {
        self.set_bool(sys::vzd_game_set_depth_buffer_enabled, enabled)
    }

    /// Enables or disables the labels buffer.
    pub fn set_labels_buffer_enabled(&mut self, enabled: bool) -> Result<()> {
        self.set_bool(sys::vzd_game_set_labels_buffer_enabled, enabled)
    }

    /// Enables or disables the automap buffer.
    pub fn set_automap_buffer_enabled(&mut self, enabled: bool) -> Result<()> {
        self.set_bool(sys::vzd_game_set_automap_buffer_enabled, enabled)
    }

    /// Enables or disables the audio buffer.
    pub fn set_audio_buffer_enabled(&mut self, enabled: bool) -> Result<()> {
        self.set_bool(sys::vzd_game_set_audio_buffer_enabled, enabled)
    }

    // ---- Action / observation spaces -------------------------------------

    /// Adds an available button (no delta max value).
    pub fn add_available_button(&mut self, button: Button) -> Result<()> {
        self.add_available_button_with_max(button, -1.0)
    }

    /// Adds an available button with a maximum delta value (use `-1.0` for the
    /// default).
    pub fn add_available_button_with_max(&mut self, button: Button, max_value: f64) -> Result<()> {
        check(unsafe {
            sys::vzd_game_add_available_button(self.raw.as_ptr(), button as c_int, max_value)
        })
    }

    /// Clears the list of available buttons.
    pub fn clear_available_buttons(&mut self) -> Result<()> {
        check(unsafe { sys::vzd_game_clear_available_buttons(self.raw.as_ptr()) })
    }

    /// Number of available buttons (the expected action vector length).
    pub fn available_buttons_size(&self) -> Result<usize> {
        let mut size: usize = 0;
        check(unsafe { sys::vzd_game_get_available_buttons_size(self.raw.as_ptr(), &mut size) })?;
        Ok(size)
    }

    /// Adds an available game variable to the state vector.
    pub fn add_available_game_variable(&mut self, variable: GameVariable) -> Result<()> {
        check(unsafe {
            sys::vzd_game_add_available_game_variable(self.raw.as_ptr(), variable as c_int)
        })
    }

    /// Clears the list of available game variables.
    pub fn clear_available_game_variables(&mut self) -> Result<()> {
        check(unsafe { sys::vzd_game_clear_available_game_variables(self.raw.as_ptr()) })
    }

    /// Reads the current value of a game variable.
    pub fn get_game_variable(&self, variable: GameVariable) -> Result<f64> {
        let mut value = 0.0f64;
        check(unsafe {
            sys::vzd_game_get_game_variable(self.raw.as_ptr(), variable as c_int, &mut value)
        })?;
        Ok(value)
    }

    // ---- Episode flow -----------------------------------------------------

    /// Starts a new episode without recording.
    pub fn new_episode(&mut self) -> Result<()> {
        check(unsafe { sys::vzd_game_new_episode(self.raw.as_ptr(), std::ptr::null()) })
    }

    /// Starts a new episode, recording it to `path`.
    pub fn new_episode_with_recording(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let c = path_to_cstring(path.as_ref())?;
        check(unsafe { sys::vzd_game_new_episode(self.raw.as_ptr(), c.as_ptr()) })
    }

    /// Whether the current state is the first of the episode.
    pub fn is_new_episode(&self) -> Result<bool> {
        self.get_bool(sys::vzd_game_is_new_episode)
    }

    /// Whether the episode has finished.
    pub fn is_episode_finished(&self) -> Result<bool> {
        self.get_bool(sys::vzd_game_is_episode_finished)
    }

    /// Whether the episode ended due to the timeout.
    pub fn is_episode_timeout_reached(&self) -> Result<bool> {
        self.get_bool(sys::vzd_game_is_episode_timeout_reached)
    }

    /// Whether the player is dead.
    pub fn is_player_dead(&self) -> Result<bool> {
        self.get_bool(sys::vzd_game_is_player_dead)
    }

    /// Respawns the player (in applicable modes).
    pub fn respawn_player(&mut self) -> Result<()> {
        check(unsafe { sys::vzd_game_respawn_player(self.raw.as_ptr()) })
    }

    // ---- Actions / rewards ------------------------------------------------

    /// Sets the action to be applied on the next [`advance_action`](Self::advance_action).
    pub fn set_action(&mut self, action: &[f64]) -> Result<()> {
        check(unsafe {
            sys::vzd_game_set_action(self.raw.as_ptr(), action.as_ptr(), action.len())
        })
    }

    /// Advances the simulation by `tics`, optionally updating the state.
    pub fn advance_action(&mut self, tics: u32, update_state: bool) -> Result<()> {
        check(unsafe {
            sys::vzd_game_advance_action(self.raw.as_ptr(), tics, update_state as c_int)
        })
    }

    /// Applies `action` for `tics` tics and returns the reward gained.
    pub fn make_action(&mut self, action: &[f64], tics: u32) -> Result<f64> {
        let mut reward = 0.0f64;
        check(unsafe {
            sys::vzd_game_make_action(
                self.raw.as_ptr(),
                action.as_ptr(),
                action.len(),
                tics,
                &mut reward,
            )
        })?;
        Ok(reward)
    }

    /// The reward from the last action.
    pub fn last_reward(&self) -> Result<f64> {
        self.get_f64(sys::vzd_game_get_last_reward)
    }

    /// The total reward accumulated this episode.
    pub fn total_reward(&self) -> Result<f64> {
        self.get_f64(sys::vzd_game_get_total_reward)
    }

    // ---- Screen info ------------------------------------------------------

    /// Screen width in pixels.
    pub fn screen_width(&self) -> Result<i32> {
        self.get_i32(sys::vzd_game_get_screen_width)
    }

    /// Screen height in pixels.
    pub fn screen_height(&self) -> Result<i32> {
        self.get_i32(sys::vzd_game_get_screen_height)
    }

    /// Number of channels in the screen buffer.
    pub fn screen_channels(&self) -> Result<i32> {
        self.get_i32(sys::vzd_game_get_screen_channels)
    }

    /// Total size of the screen buffer in bytes.
    pub fn screen_size(&self) -> Result<usize> {
        let mut value: usize = 0;
        check(unsafe { sys::vzd_game_get_screen_size(self.raw.as_ptr(), &mut value) })?;
        Ok(value)
    }

    /// The configured screen format.
    pub fn screen_format(&self) -> Result<ScreenFormat> {
        let raw = self.get_i32(sys::vzd_game_get_screen_format)?;
        ScreenFormat::from_raw(raw)
            .ok_or_else(|| Error::Engine(format!("unknown screen format {raw}")))
    }

    // ---- State ------------------------------------------------------------

    /// Returns a snapshot of the current state, or `None` if none is available
    /// (e.g. the episode has finished).
    pub fn state(&self) -> Result<Option<GameState>> {
        let mut raw: *mut sys::VzdState = std::ptr::null_mut();
        check(unsafe { sys::vzd_game_get_state(self.raw.as_ptr(), &mut raw) })?;
        Ok(NonNull::new(raw).map(GameState::from_raw))
    }

    // ---- Internal helpers -------------------------------------------------

    fn set_path(
        &mut self,
        setter: unsafe extern "C" fn(*mut sys::VzdGame, *const std::os::raw::c_char) -> sys::VzdStatus,
        path: &Path,
    ) -> Result<()> {
        let c = path_to_cstring(path)?;
        check(unsafe { setter(self.raw.as_ptr(), c.as_ptr()) })
    }

    fn set_bool(
        &mut self,
        setter: unsafe extern "C" fn(*mut sys::VzdGame, c_int) -> sys::VzdStatus,
        value: bool,
    ) -> Result<()> {
        check(unsafe { setter(self.raw.as_ptr(), value as c_int) })
    }

    fn get_bool(
        &self,
        getter: unsafe extern "C" fn(*mut sys::VzdGame, *mut c_int) -> sys::VzdStatus,
    ) -> Result<bool> {
        let mut value: c_int = 0;
        check(unsafe { getter(self.raw.as_ptr(), &mut value) })?;
        Ok(value != 0)
    }

    fn get_i32(
        &self,
        getter: unsafe extern "C" fn(*mut sys::VzdGame, *mut c_int) -> sys::VzdStatus,
    ) -> Result<i32> {
        let mut value: c_int = 0;
        check(unsafe { getter(self.raw.as_ptr(), &mut value) })?;
        Ok(value as i32)
    }

    fn get_f64(
        &self,
        getter: unsafe extern "C" fn(*mut sys::VzdGame, *mut f64) -> sys::VzdStatus,
    ) -> Result<f64> {
        let mut value = 0.0f64;
        check(unsafe { getter(self.raw.as_ptr(), &mut value) })?;
        Ok(value)
    }
}

impl Drop for DoomGame {
    fn drop(&mut self) {
        unsafe { sys::vzd_game_free(self.raw.as_ptr()) };
    }
}

/// Converts a string slice into a [`CString`], mapping interior NUL bytes to an
/// [`Error::InvalidArgument`].
fn to_cstring(value: &str) -> Result<CString> {
    CString::new(value).map_err(|e| Error::InvalidArgument(e.to_string()))
}

/// Converts a path into a [`CString`] (lossy on non-UTF-8 paths).
fn path_to_cstring(path: &Path) -> Result<CString> {
    let s = path.to_string_lossy();
    CString::new(s.as_bytes()).map_err(|e| Error::InvalidArgument(e.to_string()))
}
