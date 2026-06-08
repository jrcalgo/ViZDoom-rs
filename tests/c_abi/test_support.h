/*
 Minimal dependency-free test harness for the ViZDoom C ABI tests.

 Usage:
   #include "test_support.h"
   int main() {
       CHECK(some_condition);
       CHECK_EQ(actual, expected);
       return vzd_test::summary("my suite");
   }

 Failures are recorded (not fatal) so a run reports every problem at once.
 summary() prints a report and returns non-zero when any check failed, which
 ctest interprets as a test failure.
*/

#ifndef VZD_TEST_SUPPORT_H
#define VZD_TEST_SUPPORT_H

#include <cstdio>
#include <string>

namespace vzd_test {

inline int &failures() {
    static int count = 0;
    return count;
}

inline int &checks() {
    static int count = 0;
    return count;
}

inline void record(bool ok, const char *expr, const char *file, int line) {
    ++checks();
    if (!ok) {
        ++failures();
        std::fprintf(stderr, "FAIL: %s  (%s:%d)\n", expr, file, line);
    }
}

template <class A, class B>
inline void record_eq(const A &a, const B &b, const char *expr, const char *file, int line) {
    ++checks();
    if (!(a == b)) {
        ++failures();
        std::fprintf(stderr, "FAIL: %s  (%s:%d)\n", expr, file, line);
    }
}

inline int summary(const char *name) {
    std::fprintf(stderr, "[%s] %d checks, %d failures\n", name, checks(), failures());
    return failures() == 0 ? 0 : 1;
}

} // namespace vzd_test

#define CHECK(expr) ::vzd_test::record((expr), #expr, __FILE__, __LINE__)
#define CHECK_EQ(a, b) ::vzd_test::record_eq((a), (b), #a " == " #b, __FILE__, __LINE__)

#endif /* VZD_TEST_SUPPORT_H */
