/*
 C language ABI implementation for ViZDoom. See include/ViZDoomC.h.

 Each exported function catches all C++ exceptions at the boundary, maps them to
 a VzdStatus, and records a thread-local message for vzd_last_error_message().
*/

#include "ViZDoomC.h"

#include "ViZDoomGame.h"
#include "ViZDoomExceptions.h"
#include "ViZDoomTypes.h"

#include <new>
#include <stdexcept>
#include <string>
#include <vector>

using namespace vizdoom;

/* Opaque handle definitions. */
struct VzdGame {
    DoomGame game;
};

struct VzdState {
    GameStatePtr ptr;
};

namespace {

thread_local std::string g_lastError;

void setError(const std::string &message) {
    g_lastError = message;
}

void clearError() {
    g_lastError.clear();
}

/* Runs a callable, translating exceptions into a VzdStatus. */
template <class F>
VzdStatus guard(F &&fn) {
    try {
        clearError();
        return fn();
    } catch (const FileDoesNotExistException &e) {
        setError(e.what());
        return VZD_FILE_NOT_FOUND;
    } catch (const ViZDoomIsNotRunningException &e) {
        setError(e.what());
        return VZD_NOT_RUNNING;
    } catch (const std::invalid_argument &e) {
        setError(e.what());
        return VZD_INVALID_ARGUMENT;
    } catch (const std::exception &e) {
        setError(e.what());
        return VZD_ERROR;
    } catch (...) {
        setError("unknown error");
        return VZD_ERROR;
    }
}

} // namespace

extern "C" {

const char *vzd_last_error_message(void) {
    return g_lastError.c_str();
}

/* ---- Lifecycle -------------------------------------------------------- */

VzdStatus vzd_game_new(VzdGame **out_game) {
    if (out_game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_game = new VzdGame();
        return VZD_OK;
    });
}

void vzd_game_free(VzdGame *game) {
    delete game;
}

VzdStatus vzd_game_init(VzdGame *game, int *out_success) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        bool success = game->game.init();
        if (out_success != nullptr) *out_success = success ? 1 : 0;
        return VZD_OK;
    });
}

VzdStatus vzd_game_close(VzdGame *game) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.close();
        return VZD_OK;
    });
}

VzdStatus vzd_game_is_running(VzdGame *game, int *out_running) {
    if (game == nullptr || out_running == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_running = game->game.isRunning() ? 1 : 0;
        return VZD_OK;
    });
}

/* ---- Configuration ---------------------------------------------------- */

VzdStatus vzd_game_load_config(VzdGame *game, const char *path, int *out_success) {
    if (game == nullptr || path == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        bool success = game->game.loadConfig(path);
        if (out_success != nullptr) *out_success = success ? 1 : 0;
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_config(VzdGame *game, const char *config, int *out_success) {
    if (game == nullptr || config == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        bool success = game->game.setConfig(config);
        if (out_success != nullptr) *out_success = success ? 1 : 0;
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_vizdoom_path(VzdGame *game, const char *path) {
    if (game == nullptr || path == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setViZDoomPath(path);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_doom_game_path(VzdGame *game, const char *path) {
    if (game == nullptr || path == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setDoomGamePath(path);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_doom_scenario_path(VzdGame *game, const char *path) {
    if (game == nullptr || path == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setDoomScenarioPath(path);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_doom_map(VzdGame *game, const char *map) {
    if (game == nullptr || map == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setDoomMap(map);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_mode(VzdGame *game, int mode) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setMode(static_cast<Mode>(mode));
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_seed(VzdGame *game, uint32_t seed) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setSeed(seed);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_episode_timeout(VzdGame *game, uint32_t tics) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setEpisodeTimeout(tics);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_episode_start_time(VzdGame *game, uint32_t tics) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setEpisodeStartTime(tics);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_screen_resolution(VzdGame *game, int resolution) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setScreenResolution(static_cast<ScreenResolution>(resolution));
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_screen_format(VzdGame *game, int format) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setScreenFormat(static_cast<ScreenFormat>(format));
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_window_visible(VzdGame *game, int visible) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setWindowVisible(visible != 0);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_render_hud(VzdGame *game, int hud) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setRenderHud(hud != 0);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_living_reward(VzdGame *game, double reward) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setLivingReward(reward);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_death_penalty(VzdGame *game, double penalty) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setDeathPenalty(penalty);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_depth_buffer_enabled(VzdGame *game, int enabled) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setDepthBufferEnabled(enabled != 0);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_labels_buffer_enabled(VzdGame *game, int enabled) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setLabelsBufferEnabled(enabled != 0);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_automap_buffer_enabled(VzdGame *game, int enabled) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setAutomapBufferEnabled(enabled != 0);
        return VZD_OK;
    });
}

VzdStatus vzd_game_set_audio_buffer_enabled(VzdGame *game, int enabled) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.setAudioBufferEnabled(enabled != 0);
        return VZD_OK;
    });
}

/* ---- Action / observation spaces -------------------------------------- */

VzdStatus vzd_game_add_available_button(VzdGame *game, int button, double max_value) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.addAvailableButton(static_cast<Button>(button), max_value);
        return VZD_OK;
    });
}

VzdStatus vzd_game_clear_available_buttons(VzdGame *game) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.clearAvailableButtons();
        return VZD_OK;
    });
}

VzdStatus vzd_game_get_available_buttons_size(VzdGame *game, size_t *out_size) {
    if (game == nullptr || out_size == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_size = game->game.getAvailableButtonsSize();
        return VZD_OK;
    });
}

VzdStatus vzd_game_add_available_game_variable(VzdGame *game, int variable) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.addAvailableGameVariable(static_cast<GameVariable>(variable));
        return VZD_OK;
    });
}

VzdStatus vzd_game_clear_available_game_variables(VzdGame *game) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.clearAvailableGameVariables();
        return VZD_OK;
    });
}

VzdStatus vzd_game_get_game_variable(VzdGame *game, int variable, double *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = game->game.getGameVariable(static_cast<GameVariable>(variable));
        return VZD_OK;
    });
}

/* ---- Episode flow ----------------------------------------------------- */

VzdStatus vzd_game_new_episode(VzdGame *game, const char *recording_path) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.newEpisode(recording_path != nullptr ? std::string(recording_path) : std::string());
        return VZD_OK;
    });
}

VzdStatus vzd_game_is_new_episode(VzdGame *game, int *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = game->game.isNewEpisode() ? 1 : 0;
        return VZD_OK;
    });
}

VzdStatus vzd_game_is_episode_finished(VzdGame *game, int *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = game->game.isEpisodeFinished() ? 1 : 0;
        return VZD_OK;
    });
}

VzdStatus vzd_game_is_episode_timeout_reached(VzdGame *game, int *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = game->game.isEpisodeTimeoutReached() ? 1 : 0;
        return VZD_OK;
    });
}

VzdStatus vzd_game_is_player_dead(VzdGame *game, int *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = game->game.isPlayerDead() ? 1 : 0;
        return VZD_OK;
    });
}

VzdStatus vzd_game_respawn_player(VzdGame *game) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.respawnPlayer();
        return VZD_OK;
    });
}

/* ---- Actions / rewards ------------------------------------------------ */

VzdStatus vzd_game_set_action(VzdGame *game, const double *actions, size_t actions_len) {
    if (game == nullptr || (actions == nullptr && actions_len != 0)) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        std::vector<double> action(actions, actions + actions_len);
        game->game.setAction(action);
        return VZD_OK;
    });
}

VzdStatus vzd_game_advance_action(VzdGame *game, uint32_t tics, int update_state) {
    if (game == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        game->game.advanceAction(tics, update_state != 0);
        return VZD_OK;
    });
}

VzdStatus vzd_game_make_action(VzdGame *game, const double *actions, size_t actions_len,
                               uint32_t tics, double *out_reward) {
    if (game == nullptr || (actions == nullptr && actions_len != 0)) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        std::vector<double> action(actions, actions + actions_len);
        double reward = game->game.makeAction(action, tics);
        if (out_reward != nullptr) *out_reward = reward;
        return VZD_OK;
    });
}

VzdStatus vzd_game_get_last_reward(VzdGame *game, double *out_reward) {
    if (game == nullptr || out_reward == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_reward = game->game.getLastReward();
        return VZD_OK;
    });
}

VzdStatus vzd_game_get_total_reward(VzdGame *game, double *out_reward) {
    if (game == nullptr || out_reward == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_reward = game->game.getTotalReward();
        return VZD_OK;
    });
}

/* ---- Screen info ------------------------------------------------------ */

VzdStatus vzd_game_get_screen_width(VzdGame *game, int *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = game->game.getScreenWidth();
        return VZD_OK;
    });
}

VzdStatus vzd_game_get_screen_height(VzdGame *game, int *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = game->game.getScreenHeight();
        return VZD_OK;
    });
}

VzdStatus vzd_game_get_screen_channels(VzdGame *game, int *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = game->game.getScreenChannels();
        return VZD_OK;
    });
}

VzdStatus vzd_game_get_screen_size(VzdGame *game, size_t *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = game->game.getScreenSize();
        return VZD_OK;
    });
}

VzdStatus vzd_game_get_screen_format(VzdGame *game, int *out_value) {
    if (game == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        *out_value = static_cast<int>(game->game.getScreenFormat());
        return VZD_OK;
    });
}

/* ---- State snapshot --------------------------------------------------- */

VzdStatus vzd_game_get_state(VzdGame *game, VzdState **out_state) {
    if (game == nullptr || out_state == nullptr) return VZD_INVALID_ARGUMENT;
    return guard([&]() -> VzdStatus {
        GameStatePtr state = game->game.getState();
        if (state == nullptr) {
            *out_state = nullptr;
            return VZD_OK;
        }
        VzdState *wrapper = new VzdState();
        wrapper->ptr = state;
        *out_state = wrapper;
        return VZD_OK;
    });
}

void vzd_state_free(VzdState *state) {
    delete state;
}

VzdStatus vzd_state_number(const VzdState *state, uint32_t *out_value) {
    if (state == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    *out_value = state->ptr->number;
    return VZD_OK;
}

VzdStatus vzd_state_tic(const VzdState *state, uint32_t *out_value) {
    if (state == nullptr || out_value == nullptr) return VZD_INVALID_ARGUMENT;
    *out_value = state->ptr->tic;
    return VZD_OK;
}

VzdStatus vzd_state_game_variables(const VzdState *state, const double **out_data, size_t *out_len) {
    if (state == nullptr || out_data == nullptr || out_len == nullptr) return VZD_INVALID_ARGUMENT;
    const std::vector<double> &vars = state->ptr->gameVariables;
    *out_data = vars.empty() ? nullptr : vars.data();
    *out_len = vars.size();
    return VZD_OK;
}

namespace {

VzdStatus imageBuffer(const ImageBufferPtr &buffer, const uint8_t **out_data, size_t *out_len) {
    if (out_data == nullptr || out_len == nullptr) return VZD_INVALID_ARGUMENT;
    if (buffer == nullptr || buffer->empty()) {
        *out_data = nullptr;
        *out_len = 0;
    } else {
        *out_data = buffer->data();
        *out_len = buffer->size();
    }
    return VZD_OK;
}

} // namespace

VzdStatus vzd_state_screen_buffer(const VzdState *state, const uint8_t **out_data, size_t *out_len) {
    if (state == nullptr) return VZD_INVALID_ARGUMENT;
    return imageBuffer(state->ptr->screenBuffer, out_data, out_len);
}

VzdStatus vzd_state_depth_buffer(const VzdState *state, const uint8_t **out_data, size_t *out_len) {
    if (state == nullptr) return VZD_INVALID_ARGUMENT;
    return imageBuffer(state->ptr->depthBuffer, out_data, out_len);
}

VzdStatus vzd_state_labels_buffer(const VzdState *state, const uint8_t **out_data, size_t *out_len) {
    if (state == nullptr) return VZD_INVALID_ARGUMENT;
    return imageBuffer(state->ptr->labelsBuffer, out_data, out_len);
}

VzdStatus vzd_state_automap_buffer(const VzdState *state, const uint8_t **out_data, size_t *out_len) {
    if (state == nullptr) return VZD_INVALID_ARGUMENT;
    return imageBuffer(state->ptr->automapBuffer, out_data, out_len);
}

VzdStatus vzd_state_audio_buffer(const VzdState *state, const int16_t **out_data, size_t *out_len) {
    if (state == nullptr || out_data == nullptr || out_len == nullptr) return VZD_INVALID_ARGUMENT;
    const AudioBufferPtr &buffer = state->ptr->audioBuffer;
    if (buffer == nullptr || buffer->empty()) {
        *out_data = nullptr;
        *out_len = 0;
    } else {
        *out_data = buffer->data();
        *out_len = buffer->size();
    }
    return VZD_OK;
}

} // extern "C"
