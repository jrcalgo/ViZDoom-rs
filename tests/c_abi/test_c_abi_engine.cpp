/*
 Engine-backed tests for the ViZDoom C ABI.

 These spawn the real engine and therefore require a built `vizdoom` binary and
 scenario resources. Provide their locations via environment variables:
   VIZDOOM_BIN        directory containing the engine binary + IWAD (default ../../bin)
   VIZDOOM_SCENARIOS  directory containing scenario WADs       (default ../../scenarios)

 When VIZDOOM_BIN is unset the test self-skips (prints SKIP, returns 0) so it is
 safe to register unconditionally with ctest.
*/

#include "test_support.h"

#include "ViZDoomC.h"
#include "ViZDoomTypes.h"

#include <cstdlib>
#include <string>

using namespace vizdoom;

namespace {

std::string envOr(const char *name, const std::string &fallback) {
    const char *value = std::getenv(name);
    return (value != nullptr && value[0] != '\0') ? std::string(value) : fallback;
}

VzdGame *makeConfiguredGame(const std::string &bin, const std::string &scenarios) {
    VzdGame *game = nullptr;
    if (vzd_game_new(&game) != VZD_OK) return nullptr;

    vzd_game_set_vizdoom_path(game, (bin + "/vizdoom").c_str());
    vzd_game_set_doom_game_path(game, (bin + "/freedoom2.wad").c_str());
    vzd_game_set_doom_scenario_path(game, (scenarios + "/basic.wad").c_str());
    vzd_game_set_doom_map(game, "map01");
    vzd_game_set_screen_resolution(game, RES_160X120);
    vzd_game_set_screen_format(game, RGB24);
    vzd_game_set_window_visible(game, 0);
    vzd_game_set_mode(game, PLAYER);
    vzd_game_add_available_button(game, MOVE_LEFT, -1.0);
    vzd_game_add_available_button(game, MOVE_RIGHT, -1.0);
    vzd_game_add_available_button(game, ATTACK, -1.0);
    vzd_game_add_available_game_variable(game, AMMO2);
    vzd_game_set_episode_timeout(game, 50);
    return game;
}

} // namespace

int main() {
    const char *binEnv = std::getenv("VIZDOOM_BIN");
    if (binEnv == nullptr || binEnv[0] == '\0') {
        std::fprintf(stderr, "SKIP: VIZDOOM_BIN not set; skipping engine-backed C ABI tests\n");
        return 0;
    }

    const std::string bin = envOr("VIZDOOM_BIN", "../../bin");
    const std::string scenarios = envOr("VIZDOOM_SCENARIOS", "../../scenarios");

    VzdGame *game = makeConfiguredGame(bin, scenarios);
    CHECK(game != nullptr);
    if (game == nullptr) return vzd_test::summary("c_abi_engine");

    int success = 0;
    CHECK_EQ(vzd_game_init(game, &success), VZD_OK);
    CHECK_EQ(success, 1);

    int running = 0;
    CHECK_EQ(vzd_game_is_running(game, &running), VZD_OK);
    CHECK_EQ(running, 1);

    size_t buttons = 0;
    CHECK_EQ(vzd_game_get_available_buttons_size(game, &buttons), VZD_OK);
    CHECK_EQ(buttons, static_cast<size_t>(3));

    size_t screenSize = 0;
    CHECK_EQ(vzd_game_get_screen_size(game, &screenSize), VZD_OK);
    CHECK(screenSize > 0);

    CHECK_EQ(vzd_game_new_episode(game, nullptr), VZD_OK);

    // State while running: screen buffer length must equal the reported size,
    // and the variables vector must match the single registered variable.
    VzdState *state = nullptr;
    CHECK_EQ(vzd_game_get_state(game, &state), VZD_OK);
    CHECK(state != nullptr);
    if (state != nullptr) {
        const uint8_t *screen = nullptr;
        size_t screenLen = 0;
        CHECK_EQ(vzd_state_screen_buffer(state, &screen, &screenLen), VZD_OK);
        CHECK(screen != nullptr);
        CHECK_EQ(screenLen, screenSize);

        const double *vars = nullptr;
        size_t varsLen = 0;
        CHECK_EQ(vzd_state_game_variables(state, &vars, &varsLen), VZD_OK);
        CHECK_EQ(varsLen, static_cast<size_t>(1));

        // Buffers not enabled report null + zero, still VZD_OK.
        const uint8_t *depth = nullptr;
        size_t depthLen = 99;
        CHECK_EQ(vzd_state_depth_buffer(state, &depth, &depthLen), VZD_OK);
        CHECK(depth == nullptr);
        CHECK_EQ(depthLen, static_cast<size_t>(0));

        vzd_state_free(state);
    }

    // Reward path + stepping loop.
    double action[3] = {0.0, 1.0, 0.0};
    double reward = 0.0;
    CHECK_EQ(vzd_game_make_action(game, action, 3, 1, &reward), VZD_OK);

    int finished = 0;
    int steps = 0;
    while (vzd_game_is_episode_finished(game, &finished) == VZD_OK && finished == 0) {
        double r = 0.0;
        if (vzd_game_make_action(game, action, 3, 1, &r) != VZD_OK) break;
        ++steps;
        if (steps > 1000) break; // safety net
    }
    CHECK(steps > 0);

    double total = 0.0;
    CHECK_EQ(vzd_game_get_total_reward(game, &total), VZD_OK);

    CHECK_EQ(vzd_game_close(game), VZD_OK);
    vzd_game_free(game);

    // Separate run validating depth buffer becomes non-empty when enabled.
    VzdGame *depthGame = makeConfiguredGame(bin, scenarios);
    CHECK(depthGame != nullptr);
    if (depthGame != nullptr) {
        CHECK_EQ(vzd_game_set_depth_buffer_enabled(depthGame, 1), VZD_OK);
        int ok = 0;
        CHECK_EQ(vzd_game_init(depthGame, &ok), VZD_OK);
        CHECK_EQ(vzd_game_new_episode(depthGame, nullptr), VZD_OK);

        VzdState *s = nullptr;
        CHECK_EQ(vzd_game_get_state(depthGame, &s), VZD_OK);
        if (s != nullptr) {
            const uint8_t *depth = nullptr;
            size_t depthLen = 0;
            CHECK_EQ(vzd_state_depth_buffer(s, &depth, &depthLen), VZD_OK);
            CHECK(depth != nullptr);
            CHECK(depthLen > 0);
            vzd_state_free(s);
        }
        vzd_game_close(depthGame);
        vzd_game_free(depthGame);
    }

    return vzd_test::summary("c_abi_engine");
}
