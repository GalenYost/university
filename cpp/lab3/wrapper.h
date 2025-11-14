#pragma once

#include "tree.h"

template <typename T> class AccessWrapper {
    T *ptr;

  public:
    explicit AccessWrapper(T *p);
    ~AccessWrapper();

    T *operator->();
    const T *operator->() const;

    T &operator*();
    const T &operator*() const;
};
