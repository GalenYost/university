#include "tests.h"

static void array_test() {
    INT_ARRAY arr;
    INT_ARRAY_init(&arr, 1);

    INT_ARRAY_push(&arr, 10);

    assert_inline(arr.size == 1, "Array length is not 1", "Array length is 1");
    assert_inline(INT_ARRAY_get(&arr, 0) != NULL, "First element is NULL",
                  "First element is 10");
}

void run_tests() { array_test(); }
