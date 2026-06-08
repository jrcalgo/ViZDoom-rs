# ViZDoom C ABI tests.
#
# Built only when BUILD_C_ABI_TESTS=ON (see the root CMakeLists.txt). Links the
# static core library and its private dependencies (defined at root scope in
# VIZDOOM_LIBS). Two executables:
#   c_abi_contract  engine-free contract tests (run anywhere)
#   c_abi_engine    engine-backed tests (self-skips without VIZDOOM_BIN)

set(VIZDOOM_C_ABI_TEST_LIBS libvizdoom_static ${VIZDOOM_LIBS})

add_executable(c_abi_contract test_c_abi_contract.cpp)
target_include_directories(c_abi_contract PRIVATE
    ${VIZDOOM_INCLUDE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(c_abi_contract PRIVATE ${VIZDOOM_C_ABI_TEST_LIBS})

add_executable(c_abi_engine test_c_abi_engine.cpp)
target_include_directories(c_abi_engine PRIVATE
    ${VIZDOOM_INCLUDE_DIR}
    ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(c_abi_engine PRIVATE ${VIZDOOM_C_ABI_TEST_LIBS})

add_test(NAME c_abi_contract COMMAND c_abi_contract)

add_test(NAME c_abi_engine COMMAND c_abi_engine)
set_tests_properties(c_abi_engine PROPERTIES LABELS "engine")
