/*
 Engine-free contract tests for the ViZDoom C ABI (include/ViZDoomC.h).

 These exercise the FFI boundary without spawning the engine: a DoomGame can be
 constructed and configured, and every fallible entry point has a well-defined
 status without a running engine. No Boost-backed engine process is started.
*/

#include "test_support.h"

#include "ViZDoomC.h"
#include "ViZDoomTypes.h"

#include <cstring>
#include <string>
#include <thread>

using namespace vizdoom;

/* Compile-time enum parity: the integer values crossing the ABI must match the
   Rust mirrors in crates/vizdoom/src/enums.rs. If the C++ enums ever shift,
   this fails the build rather than silently corrupting actions/observations. */
static_assert(ATTACK == 0, "Button::Attack");
static_assert(MOVE_RIGHT == 10, "Button::MoveRight");
static_assert(MOVE_LEFT == 11, "Button::MoveLeft");
static_assert(MOVE_FORWARD == 13, "Button::MoveForward");
static_assert(MOVE_UP_DOWN_DELTA == 42, "Button::MoveUpDownDelta");
static_assert(KILLCOUNT == 0, "GameVariable::KillCount");
static_assert(HEALTH == 9, "GameVariable::Health");
static_assert(AMMO2 == 19, "GameVariable::Ammo2");
static_assert(POSITION_X == 37, "GameVariable::PositionX");
static_assert(PLAYER16_FRAGCOUNT == 71, "GameVariable::Player16FragCount");
static_assert(USER1 == 72, "GameVariable::User1");
static_assert(USER60 == 131, "GameVariable::User60");
static_assert(PLAYER == 0 && ASYNC_SPECTATOR == 3, "Mode");
static_assert(CRCGCB == 0 && DOOM_256_COLORS8 == 9, "ScreenFormat");
static_assert(RES_160X120 == 0 && RES_1920X1080 == 35, "ScreenResolution");

static void test_lifecycle() {
    VzdGame *game = nullptr;
    CHECK_EQ(vzd_game_new(&game), VZD_OK);
    CHECK(game != nullptr);

    int running = -1;
    CHECK_EQ(vzd_game_is_running(game, &running), VZD_OK);
    CHECK_EQ(running, 0);

    vzd_game_free(game);

    // new() rejects a null out-param.
    CHECK_EQ(vzd_game_new(nullptr), VZD_INVALID_ARGUMENT);
    // free() on null is a no-op (must not crash).
    vzd_game_free(nullptr);
}

static void test_null_handles() {
    int out = 0;
    double dout = 0.0;
    size_t sout = 0;
    const uint8_t *bytes = nullptr;
    size_t blen = 0;

    CHECK_EQ(vzd_game_init(nullptr, &out), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_close(nullptr), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_is_running(nullptr, &out), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_load_config(nullptr, "x", &out), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_set_doom_map(nullptr, "map01"), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_set_mode(nullptr, 0), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_add_available_button(nullptr, 0, -1.0), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_get_available_buttons_size(nullptr, &sout), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_new_episode(nullptr, nullptr), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_make_action(nullptr, nullptr, 0, 1, &dout), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_get_state(nullptr, nullptr), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_get_screen_size(nullptr, &sout), VZD_INVALID_ARGUMENT);

    // Null state handle / out-params.
    uint32_t u32 = 0;
    CHECK_EQ(vzd_state_number(nullptr, &u32), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_state_screen_buffer(nullptr, &bytes, &blen), VZD_INVALID_ARGUMENT);
    vzd_state_free(nullptr); // no-op, must not crash
}

static void test_null_out_params() {
    VzdGame *game = nullptr;
    CHECK_EQ(vzd_game_new(&game), VZD_OK);

    CHECK_EQ(vzd_game_is_running(game, nullptr), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_get_available_buttons_size(game, nullptr), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_get_screen_size(game, nullptr), VZD_INVALID_ARGUMENT);
    CHECK_EQ(vzd_game_get_state(game, nullptr), VZD_INVALID_ARGUMENT);

    vzd_game_free(game);
}

static void test_not_running_mapping() {
    VzdGame *game = nullptr;
    CHECK_EQ(vzd_game_new(&game), VZD_OK);

    int out = 0;
    double reward = 0.0;
    double var = 0.0;
    VzdState *state = nullptr;
    double action[3] = {0.0, 0.0, 0.0};

    // Operations that require a running engine must map to VZD_NOT_RUNNING.
    CHECK_EQ(vzd_game_get_state(game, &state), VZD_NOT_RUNNING);
    CHECK_EQ(vzd_game_make_action(game, action, 3, 1, &reward), VZD_NOT_RUNNING);
    CHECK_EQ(vzd_game_set_action(game, action, 3), VZD_NOT_RUNNING);
    CHECK_EQ(vzd_game_advance_action(game, 1, 1), VZD_NOT_RUNNING);
    CHECK_EQ(vzd_game_get_game_variable(game, KILLCOUNT, &var), VZD_NOT_RUNNING);
    CHECK_EQ(vzd_game_get_last_reward(game, &reward), VZD_NOT_RUNNING);
    CHECK_EQ(vzd_game_respawn_player(game), VZD_NOT_RUNNING);
    CHECK_EQ(vzd_game_is_episode_finished(game, &out), VZD_NOT_RUNNING);
    CHECK_EQ(vzd_game_is_player_dead(game, &out), VZD_NOT_RUNNING);

    vzd_game_free(game);
}

static void test_arg_ordering() {
    VzdGame *game = nullptr;
    CHECK_EQ(vzd_game_new(&game), VZD_OK);

    // Null data with non-zero length is rejected before reaching the engine.
    CHECK_EQ(vzd_game_set_action(game, nullptr, 5), VZD_INVALID_ARGUMENT);
    // Null data with zero length passes the guard, then hits not-running.
    CHECK_EQ(vzd_game_set_action(game, nullptr, 0), VZD_NOT_RUNNING);

    vzd_game_free(game);
}

static void test_engine_free_config() {
    VzdGame *game = nullptr;
    CHECK_EQ(vzd_game_new(&game), VZD_OK);

    // Config setters that don't require a running engine succeed.
    CHECK_EQ(vzd_game_set_doom_map(game, "map01"), VZD_OK);
    CHECK_EQ(vzd_game_set_mode(game, PLAYER), VZD_OK);
    CHECK_EQ(vzd_game_set_seed(game, 42), VZD_OK);
    CHECK_EQ(vzd_game_set_episode_timeout(game, 200), VZD_OK);
    CHECK_EQ(vzd_game_set_screen_resolution(game, RES_320X240), VZD_OK);
    CHECK_EQ(vzd_game_set_screen_format(game, RGB24), VZD_OK);
    CHECK_EQ(vzd_game_set_window_visible(game, 0), VZD_OK);
    CHECK_EQ(vzd_game_set_living_reward(game, 1.0), VZD_OK);

    // Button / variable list management.
    size_t size = 99;
    CHECK_EQ(vzd_game_clear_available_buttons(game), VZD_OK);
    CHECK_EQ(vzd_game_get_available_buttons_size(game, &size), VZD_OK);
    CHECK_EQ(size, static_cast<size_t>(0));

    CHECK_EQ(vzd_game_add_available_button(game, MOVE_LEFT, -1.0), VZD_OK);
    CHECK_EQ(vzd_game_add_available_button(game, MOVE_RIGHT, -1.0), VZD_OK);
    CHECK_EQ(vzd_game_add_available_button(game, ATTACK, -1.0), VZD_OK);
    CHECK_EQ(vzd_game_get_available_buttons_size(game, &size), VZD_OK);
    CHECK_EQ(size, static_cast<size_t>(3));

    CHECK_EQ(vzd_game_clear_available_buttons(game), VZD_OK);
    CHECK_EQ(vzd_game_get_available_buttons_size(game, &size), VZD_OK);
    CHECK_EQ(size, static_cast<size_t>(0));

    CHECK_EQ(vzd_game_add_available_game_variable(game, AMMO2), VZD_OK);
    CHECK_EQ(vzd_game_clear_available_game_variables(game), VZD_OK);

    vzd_game_free(game);
}

static void test_file_not_found() {
    VzdGame *game = nullptr;
    CHECK_EQ(vzd_game_new(&game), VZD_OK);

    int success = -1;
    // A missing config path throws FileDoesNotExistException -> VZD_FILE_NOT_FOUND.
    CHECK_EQ(vzd_game_load_config(game, "this/path/does/not/exist.cfg", &success),
             VZD_FILE_NOT_FOUND);

    vzd_game_free(game);
}

static void test_error_message_semantics() {
    VzdGame *game = nullptr;
    CHECK_EQ(vzd_game_new(&game), VZD_OK);

    // Trigger a failure, expect a non-empty message.
    double reward = 0.0;
    double action[1] = {0.0};
    CHECK_EQ(vzd_game_make_action(game, action, 1, 1, &reward), VZD_NOT_RUNNING);
    CHECK(std::strlen(vzd_last_error_message()) > 0);

    // A subsequent success clears the message.
    CHECK_EQ(vzd_game_set_mode(game, PLAYER), VZD_OK);
    CHECK_EQ(std::strlen(vzd_last_error_message()), static_cast<size_t>(0));

    vzd_game_free(game);
}

static void test_error_message_thread_local() {
    // Each thread keeps its own last-error. Fail on a worker thread, succeed on
    // the main thread, and confirm the worker's message did not leak here.
    VzdGame *mainGame = nullptr;
    CHECK_EQ(vzd_game_new(&mainGame), VZD_OK);
    CHECK_EQ(vzd_game_set_mode(mainGame, PLAYER), VZD_OK); // clears main-thread error

    bool workerSawMessage = false;
    std::thread worker([&]() {
        VzdGame *g = nullptr;
        if (vzd_game_new(&g) != VZD_OK) return;
        double reward = 0.0;
        double action[1] = {0.0};
        vzd_game_make_action(g, action, 1, 1, &reward); // fails -> sets worker-local msg
        workerSawMessage = std::strlen(vzd_last_error_message()) > 0;
        vzd_game_free(g);
    });
    worker.join();

    CHECK(workerSawMessage);
    // Main thread error remains clear despite the worker's failure.
    CHECK_EQ(std::strlen(vzd_last_error_message()), static_cast<size_t>(0));

    vzd_game_free(mainGame);
}

int main() {
    test_lifecycle();
    test_null_handles();
    test_null_out_params();
    test_not_running_mapping();
    test_arg_ordering();
    test_engine_free_config();
    test_file_not_found();
    test_error_message_semantics();
    test_error_message_thread_local();
    return vzd_test::summary("c_abi_contract");
}
