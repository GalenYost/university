#pragma once

template <typename T> class AccessWrapper {
    T *ptr;

  public:
    explicit AccessWrapper(T *p) : ptr(p) {}
    ~AccessWrapper() = default;

    T *operator->() { return ptr; }
    const T *operator->() const { return ptr; }

    T &operator*() { return *ptr; }
    const T &operator*() const { return *ptr; }
};
