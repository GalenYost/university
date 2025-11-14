#include "wrapper.h"

template <typename T> AccessWrapper<T>::AccessWrapper(T *p) : ptr(p) {}
template <typename T> AccessWrapper<T>::~AccessWrapper() = default;

template <typename T> T *AccessWrapper<T>::operator->() { return ptr; }
template <typename T> const T *AccessWrapper<T>::operator->() const {
    return ptr;
}

template <typename T> T &AccessWrapper<T>::operator*() { return *ptr; }
template <typename T> const T &AccessWrapper<T>::operator*() const {
    return *ptr;
}

template class AccessWrapper<BinaryTree<int>>;
