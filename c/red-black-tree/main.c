#ifdef TEST_MODE
#include "tests.h"
#endif

#include <stdio.h>

int main(void) {
#ifdef TEST_MODE
    run_tests();
#endif
#ifndef TEST_MODE
    printf("Hello, world!\n");
#endif
    return 0;
}
