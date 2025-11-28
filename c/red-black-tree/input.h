#ifndef INPUT_H
#define INPUT_H

#include "types.h"

struct InputBufValue {
    Pair value;
    Callback cb;
};

typedef struct {
    char *inputPrompt;
    InputBufValueMap options;
} InputBuffer;

#endif
