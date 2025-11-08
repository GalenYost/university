#include "vec.h"
#include "input.h"
#include "tree.h"

template <typename T>
Vector<T>::Vector() : vec(nullptr), capacity(0), count(0) {}

template <typename T> Vector<T>::~Vector() { clear(); }

template <typename T> void Vector<T>::clear() {
    delete[] vec;
    capacity = 0;
    count = 0;
}

template <typename T> void Vector<T>::push(const T &el) {
    if (count == capacity) {
        unsigned newCap = capacity ? capacity * 2 : 4;
        T *newVec = new T[newCap];

        for (unsigned i = 0; i < count; i++) newVec[i] = vec[i];
        delete[] vec;

        vec = newVec;
        capacity = newCap;
    }
    vec[count++] = el;
}
template <typename T> void Vector<T>::push(const T &el, unsigned idx) {
    if (idx > count) idx = count;

    if (count == capacity) {
        unsigned newCap = capacity ? capacity * 2 : 4;
        T *newVec = new T[newCap];

        for (unsigned i = 0; i < count; i++) newVec[i] = vec[i];
        delete[] vec;

        vec = newVec;
        capacity = newCap;
    }

    for (unsigned i = count; i > idx; i--) vec[i] = vec[i - 1];

    vec[idx] = el;
    count++;
}

template <typename T> T Vector<T>::pop() {
    T el = vec[--count];
    return el;
}

template <typename T> T *Vector<T>::get(unsigned idx) const {
    if (idx >= count) return nullptr;
    return &vec[idx];
}

template <typename T> unsigned Vector<T>::len() const { return count; }
template <typename T> unsigned Vector<T>::cap() const { return capacity; }

template class Vector<int>;
template class Vector<char>;

template class Vector<Pair>;

template class Vector<std::string>;

template class Vector<Node<int> *>;
template class Vector<Node<std::string> *>;
template class Vector<Node<char> *>;
