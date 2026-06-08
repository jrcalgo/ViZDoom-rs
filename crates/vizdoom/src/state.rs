//! Safe wrapper around a ViZDoom game-state snapshot.

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

use vizdoom_sys as sys;

use crate::error::{check, Result};

/// An owned snapshot of the game state at a single tic.
///
/// Buffer accessors borrow memory owned by this snapshot, so the returned
/// slices are tied to the lifetime of the `GameState`. The underlying memory is
/// released when the `GameState` is dropped.
pub struct GameState {
    raw: NonNull<sys::VzdState>,
    // The snapshot owns C++ state that is not safe to touch from multiple
    // threads simultaneously.
    _not_send_sync: PhantomData<*const ()>,
}

impl GameState {
    /// Wraps a non-null raw state pointer. The caller transfers ownership.
    pub(crate) fn from_raw(raw: NonNull<sys::VzdState>) -> Self {
        GameState {
            raw,
            _not_send_sync: PhantomData,
        }
    }

    /// Sequential state number within the episode.
    pub fn number(&self) -> Result<u32> {
        let mut value = 0u32;
        check(unsafe { sys::vzd_state_number(self.raw.as_ptr(), &mut value) })?;
        Ok(value)
    }

    /// Engine tic at which this state was captured.
    pub fn tic(&self) -> Result<u32> {
        let mut value = 0u32;
        check(unsafe { sys::vzd_state_tic(self.raw.as_ptr(), &mut value) })?;
        Ok(value)
    }

    /// The available game variables, in the order they were registered.
    pub fn game_variables(&self) -> Result<&[f64]> {
        let mut data: *const f64 = std::ptr::null();
        let mut len: usize = 0;
        check(unsafe { sys::vzd_state_game_variables(self.raw.as_ptr(), &mut data, &mut len) })?;
        Ok(slice_from_parts(data, len))
    }

    /// The main screen buffer. Layout depends on the configured screen format.
    pub fn screen_buffer(&self) -> Result<&[u8]> {
        self.image_buffer(sys::vzd_state_screen_buffer)
    }

    /// The depth buffer, or an empty slice when not enabled.
    pub fn depth_buffer(&self) -> Result<&[u8]> {
        self.image_buffer(sys::vzd_state_depth_buffer)
    }

    /// The labels buffer, or an empty slice when not enabled.
    pub fn labels_buffer(&self) -> Result<&[u8]> {
        self.image_buffer(sys::vzd_state_labels_buffer)
    }

    /// The automap buffer, or an empty slice when not enabled.
    pub fn automap_buffer(&self) -> Result<&[u8]> {
        self.image_buffer(sys::vzd_state_automap_buffer)
    }

    /// The audio buffer (interleaved stereo i16), or empty when not enabled.
    pub fn audio_buffer(&self) -> Result<&[i16]> {
        let mut data: *const i16 = std::ptr::null();
        let mut len: usize = 0;
        check(unsafe { sys::vzd_state_audio_buffer(self.raw.as_ptr(), &mut data, &mut len) })?;
        Ok(slice_from_parts(data, len))
    }

    fn image_buffer(
        &self,
        accessor: unsafe extern "C" fn(
            *const sys::VzdState,
            *mut *const u8,
            *mut usize,
        ) -> sys::VzdStatus,
    ) -> Result<&[u8]> {
        let mut data: *const u8 = std::ptr::null();
        let mut len: usize = 0;
        check(unsafe { accessor(self.raw.as_ptr(), &mut data, &mut len) })?;
        Ok(slice_from_parts(data, len))
    }
}

impl Drop for GameState {
    fn drop(&mut self) {
        unsafe { sys::vzd_state_free(self.raw.as_ptr()) };
    }
}

/// Builds a slice from a (possibly null) pointer/length pair returned by the C
/// ABI. A null pointer (buffer not enabled) yields an empty slice.
fn slice_from_parts<'a, T>(data: *const T, len: usize) -> &'a [T] {
    if data.is_null() || len == 0 {
        &[]
    } else {
        // SAFETY: the C ABI guarantees `data` points to `len` valid elements
        // owned by the snapshot, living at least as long as `&self`.
        unsafe { slice::from_raw_parts(data, len) }
    }
}
