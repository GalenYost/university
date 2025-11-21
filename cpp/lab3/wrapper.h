#pragma once

#include "tree.h"

template <typename T> class AccessWrapper {
    T *ptr;
    unsigned count = 0;

  public:
    AccessWrapper(T *p);
    ~AccessWrapper();

    T *operator->();
    const T *operator->() const;

    T &operator*();
    const T &operator*() const;

    unsigned current_count() const;
};
