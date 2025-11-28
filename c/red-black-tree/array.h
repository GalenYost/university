#ifndef ARRAY_H
#define ARRAY_H

#include <stddef.h>
#include <stdlib.h>

#define DECLARE_ARRAY_TYPE(T, Name)                                            \
    typedef struct {                                                           \
        T *data;                                                               \
        size_t size;                                                           \
        size_t cap;                                                            \
    } Name;                                                                    \
                                                                               \
    void Name##_init(Name *arr, size_t cap);                                   \
    void Name##_expand(Name *arr);                                             \
    void Name##_push(Name *arr, T value);                                      \
    void Name##_destroy(Name *arr);                                            \
    T *Name##_get(Name *arr, unsigned index);

#endif
