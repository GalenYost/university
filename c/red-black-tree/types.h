#ifndef TYPES_H
#define TYPES_H

#include "array.h"

typedef void (*fn)(void *env);

typedef struct {
    void *env;
    fn func;
} Callback;

typedef struct {
} Pair;

typedef struct InputBufValue InputBufValue;

DECLARE_ARRAY_TYPE(int, INT_ARRAY);
DECLARE_ARRAY_TYPE(InputBufValue, InputBufValueMap);

#endif
