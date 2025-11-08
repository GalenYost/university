#pragma once

#include "log.h"

template <typename T> class Vector {
  private:
    T *vec;
    unsigned capacity;
    unsigned count;

    void clear();

  public:
    Vector();
    ~Vector();

    void push(const T &el);
    void push(const T &el, unsigned idx);

    T pop();
    T *get(unsigned idx) const;

    unsigned len() const;
    unsigned cap() const;
};
