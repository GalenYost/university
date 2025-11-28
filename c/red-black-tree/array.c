#include "array.h"

#define DEFINE_ARRAY_TYPE(T, Name)                                             \
    void Name##_init(Name *arr, size_t cap) {                                  \
        if (cap == 0) cap = 1;                                                 \
        arr->data = malloc(sizeof(T) * cap);                                   \
        arr->size = 0;                                                         \
        arr->cap = cap;                                                        \
    }                                                                          \
                                                                               \
    void Name##_expand(Name *arr) {                                            \
        arr->cap *= 2;                                                         \
        arr->data = realloc(arr->data, sizeof(T) * arr->cap);                  \
    }                                                                          \
                                                                               \
    T *Name##_get(Name *arr, unsigned index) {                                 \
        if (index >= arr->size) { return NULL; }                               \
        return &arr->data[index];                                              \
    }                                                                          \
                                                                               \
    void Name##_push(Name *arr, T value) {                                     \
        if (arr->size == arr->cap) Name##_expand(arr);                         \
        arr->data[arr->size++] = value;                                        \
    }                                                                          \
                                                                               \
    void Name##_destroy(Name *arr) { free(arr->data); }
