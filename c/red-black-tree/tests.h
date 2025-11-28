#ifndef TESTS_H
#define TESTS_H

#include "array.h"
#include "log.h"

inline void assert_inline(bool expr, const char *e_msg, const char *s_msg) {
    if (!expr) {
        log_error("Assertion fail: %s", e_msg);
    } else {
        log_info("Test passed: %s", s_msg);
    }
}

void run_tests();

#endif
