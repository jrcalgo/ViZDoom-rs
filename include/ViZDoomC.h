/*
 C language ABI for ViZDoom.

 This header exposes a flat, extern "C" surface over vizdoom::DoomGame so that
 non-C++ callers (e.g. Rust via FFI) can drive ViZDoom without depending on the
 unstable C++ ABI (std::string, std::vector, std::shared_ptr, exceptions).

 Conventions:
   - All handles are opaque pointers, owned by the caller and released with the
     matching *_free function.
   - Every fallible function returns a VzdStatus. C++ exceptions are caught at
     the boundary and never cross into the caller. On a non-OK status a human
     readable message is available via vzd_last_error_message() (thread-local).
   - Out-parameters are written only on VZD_OK unless documented otherwise.
*/

#ifndef __VIZDOOM_C_H__
#define __VIZDOOM_C_H__

#include <stddef.h>
#include <stdint.h>

#if defined(_WIN32)
    #define VZD_EXPORT __declspec(dllexport)
#else
    #define VZD_EXPORT __attribute__((visibility("default")))
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef struct VzdGame VzdGame;
typedef struct VzdState VzdState;

typedef enum VzdStatus {
    VZD_OK = 0,
    VZD_ERROR = 1,            /* generic ViZDoom/engine error */
    VZD_INVALID_ARGUMENT = 2, /* null handle or bad pointer/length */
    VZD_NOT_RUNNING = 3,      /* operation needs a running game */
    VZD_FILE_NOT_FOUND = 4    /* config/resource file missing */
} VzdStatus;

/* Returns a thread-local message describing the most recent error on this
   thread, or an empty string if none. The pointer is valid until the next
   failing call on the same thread. Never returns NULL. */
VZD_EXPORT const char *vzd_last_error_message(void);

/* ---- Lifecycle -------------------------------------------------------- */

VZD_EXPORT VzdStatus vzd_game_new(VzdGame **out_game);
VZD_EXPORT void vzd_game_free(VzdGame *game);

VZD_EXPORT VzdStatus vzd_game_init(VzdGame *game, int *out_success);
VZD_EXPORT VzdStatus vzd_game_close(VzdGame *game);
VZD_EXPORT VzdStatus vzd_game_is_running(VzdGame *game, int *out_running);

/* ---- Configuration ---------------------------------------------------- */

VZD_EXPORT VzdStatus vzd_game_load_config(VzdGame *game, const char *path, int *out_success);
VZD_EXPORT VzdStatus vzd_game_set_config(VzdGame *game, const char *config, int *out_success);

VZD_EXPORT VzdStatus vzd_game_set_vizdoom_path(VzdGame *game, const char *path);
VZD_EXPORT VzdStatus vzd_game_set_doom_game_path(VzdGame *game, const char *path);
VZD_EXPORT VzdStatus vzd_game_set_doom_scenario_path(VzdGame *game, const char *path);
VZD_EXPORT VzdStatus vzd_game_set_doom_map(VzdGame *game, const char *map);

VZD_EXPORT VzdStatus vzd_game_set_mode(VzdGame *game, int mode);
VZD_EXPORT VzdStatus vzd_game_set_seed(VzdGame *game, uint32_t seed);
VZD_EXPORT VzdStatus vzd_game_set_episode_timeout(VzdGame *game, uint32_t tics);
VZD_EXPORT VzdStatus vzd_game_set_episode_start_time(VzdGame *game, uint32_t tics);

VZD_EXPORT VzdStatus vzd_game_set_screen_resolution(VzdGame *game, int resolution);
VZD_EXPORT VzdStatus vzd_game_set_screen_format(VzdGame *game, int format);
VZD_EXPORT VzdStatus vzd_game_set_window_visible(VzdGame *game, int visible);
VZD_EXPORT VzdStatus vzd_game_set_render_hud(VzdGame *game, int hud);

VZD_EXPORT VzdStatus vzd_game_set_living_reward(VzdGame *game, double reward);
VZD_EXPORT VzdStatus vzd_game_set_death_penalty(VzdGame *game, double penalty);

/* Optional buffers (off by default). */
VZD_EXPORT VzdStatus vzd_game_set_depth_buffer_enabled(VzdGame *game, int enabled);
VZD_EXPORT VzdStatus vzd_game_set_labels_buffer_enabled(VzdGame *game, int enabled);
VZD_EXPORT VzdStatus vzd_game_set_automap_buffer_enabled(VzdGame *game, int enabled);
VZD_EXPORT VzdStatus vzd_game_set_audio_buffer_enabled(VzdGame *game, int enabled);

/* ---- Action / observation spaces -------------------------------------- */

VZD_EXPORT VzdStatus vzd_game_add_available_button(VzdGame *game, int button, double max_value);
VZD_EXPORT VzdStatus vzd_game_clear_available_buttons(VzdGame *game);
VZD_EXPORT VzdStatus vzd_game_get_available_buttons_size(VzdGame *game, size_t *out_size);

VZD_EXPORT VzdStatus vzd_game_add_available_game_variable(VzdGame *game, int variable);
VZD_EXPORT VzdStatus vzd_game_clear_available_game_variables(VzdGame *game);
VZD_EXPORT VzdStatus vzd_game_get_game_variable(VzdGame *game, int variable, double *out_value);

/* ---- Episode flow ----------------------------------------------------- */

/* recording_path may be NULL or "" to disable recording. */
VZD_EXPORT VzdStatus vzd_game_new_episode(VzdGame *game, const char *recording_path);
VZD_EXPORT VzdStatus vzd_game_is_new_episode(VzdGame *game, int *out_value);
VZD_EXPORT VzdStatus vzd_game_is_episode_finished(VzdGame *game, int *out_value);
VZD_EXPORT VzdStatus vzd_game_is_episode_timeout_reached(VzdGame *game, int *out_value);
VZD_EXPORT VzdStatus vzd_game_is_player_dead(VzdGame *game, int *out_value);
VZD_EXPORT VzdStatus vzd_game_respawn_player(VzdGame *game);

/* ---- Actions / rewards ------------------------------------------------ */

VZD_EXPORT VzdStatus vzd_game_set_action(VzdGame *game, const double *actions, size_t actions_len);
VZD_EXPORT VzdStatus vzd_game_advance_action(VzdGame *game, uint32_t tics, int update_state);
VZD_EXPORT VzdStatus vzd_game_make_action(VzdGame *game, const double *actions, size_t actions_len,
                                          uint32_t tics, double *out_reward);
VZD_EXPORT VzdStatus vzd_game_get_last_reward(VzdGame *game, double *out_reward);
VZD_EXPORT VzdStatus vzd_game_get_total_reward(VzdGame *game, double *out_reward);

/* ---- Screen info ------------------------------------------------------ */

VZD_EXPORT VzdStatus vzd_game_get_screen_width(VzdGame *game, int *out_value);
VZD_EXPORT VzdStatus vzd_game_get_screen_height(VzdGame *game, int *out_value);
VZD_EXPORT VzdStatus vzd_game_get_screen_channels(VzdGame *game, int *out_value);
VZD_EXPORT VzdStatus vzd_game_get_screen_size(VzdGame *game, size_t *out_value);
VZD_EXPORT VzdStatus vzd_game_get_screen_format(VzdGame *game, int *out_value);

/* ---- State snapshot --------------------------------------------------- */

/* Allocates a snapshot of the current game state. On VZD_OK, *out_state is
   either a new handle (free with vzd_state_free) or NULL when no state is
   available (e.g. episode finished). */
VZD_EXPORT VzdStatus vzd_game_get_state(VzdGame *game, VzdState **out_state);
VZD_EXPORT void vzd_state_free(VzdState *state);

VZD_EXPORT VzdStatus vzd_state_number(const VzdState *state, uint32_t *out_value);
VZD_EXPORT VzdStatus vzd_state_tic(const VzdState *state, uint32_t *out_value);

/* Buffer accessors borrow memory owned by the state snapshot; the pointers are
   valid until vzd_state_free. When a buffer is not enabled, *out_data is set to
   NULL and *out_len to 0 (still VZD_OK). */
VZD_EXPORT VzdStatus vzd_state_game_variables(const VzdState *state, const double **out_data, size_t *out_len);
VZD_EXPORT VzdStatus vzd_state_screen_buffer(const VzdState *state, const uint8_t **out_data, size_t *out_len);
VZD_EXPORT VzdStatus vzd_state_depth_buffer(const VzdState *state, const uint8_t **out_data, size_t *out_len);
VZD_EXPORT VzdStatus vzd_state_labels_buffer(const VzdState *state, const uint8_t **out_data, size_t *out_len);
VZD_EXPORT VzdStatus vzd_state_automap_buffer(const VzdState *state, const uint8_t **out_data, size_t *out_len);
VZD_EXPORT VzdStatus vzd_state_audio_buffer(const VzdState *state, const int16_t **out_data, size_t *out_len);

#ifdef __cplusplus
}
#endif

#endif /* __VIZDOOM_C_H__ */
