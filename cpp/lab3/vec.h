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
    Vector(const Vector &other);
    Vector &operator=(const Vector &other);

    T &operator[](unsigned i);
    const T &operator[](unsigned i) const;

    void push(const T &el);
    void push(const T &el, unsigned idx);

    T pop();

    T *get(unsigned idx) const;

    unsigned len() const;
    unsigned cap() const;
    bool empty() const;
};
