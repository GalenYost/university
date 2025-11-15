#include "vec.h"
#include "input.h"
#include "tree.h"

template <typename T>
Vector<T>::Vector() : vec(nullptr), capacity(0), count(0) {}

template <typename T> Vector<T>::~Vector() { clear(); }

template <typename T> Vector<T>::Vector(const Vector &other) {
    count = other.count;
    capacity = other.capacity;
    vec = new T[capacity];
    for (unsigned i = 0; i < count; ++i) vec[i] = other.vec[i];
}

template <typename T> Vector<T> &Vector<T>::operator=(const Vector &other) {
    if (this != &other) {
        delete[] vec;
        count = other.count;
        capacity = other.capacity;
        vec = new T[capacity];
        for (unsigned i = 0; i < count; ++i) vec[i] = other.vec[i];
    }
    return *this;
}

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

template <typename T> T &Vector<T>::operator[](unsigned i) { return vec[i]; }
template <typename T> const T &Vector<T>::operator[](unsigned i) const {
    return vec[i];
}

template <typename T> T *Vector<T>::get(unsigned idx) const {
    if (idx >= count) return nullptr;
    return &vec[idx];
}

template <typename T> unsigned Vector<T>::len() const { return count; }
template <typename T> unsigned Vector<T>::cap() const { return capacity; }
template <typename T> bool Vector<T>::empty() const { return count == 0; }

template class Vector<int>;
template class Vector<Pair>;
template class Vector<std::string>;
template class Vector<Node<int> *>;

#ifdef __cplusplus
extern "C" {

Vector<int> *create_vector_int() { return new Vector<int>(); }
Vector<Pair> *create_vector_pair() { return new Vector<Pair>(); }
Vector<std::string> *create_vector_str() { return new Vector<std::string>(); }
Vector<Node<int> *> *create_vector_node() { return new Vector<Node<int> *>(); }

void destroy_vec_int(Vector<int> *vec) { delete vec; }
void destroy_vec_pair(Vector<Pair> *vec) { delete vec; }
void destroy_vec_str(Vector<std::string> *vec) { delete vec; }
void destroy_vec_node_int(Vector<Node<int> *> *vec) { delete vec; }

void push_int(Vector<int> *vec, int val) { vec->push(val); }
void push_pair(Vector<Pair> *vec, Pair val) { vec->push(val); }
void push_str(Vector<std::string> *vec, std::string val) { vec->push(val); }
void push_node_int(Vector<Node<int> *> *vec, Node<int> *val) { vec->push(val); }

int pop_int(Vector<int> *vec) { return vec->pop(); }
void pop_pair(Vector<Pair> *vec) { vec->pop(); }
void pop_str(Vector<std::string> *vec) { vec->pop(); }
Node<int> *pop_node_int(Vector<Node<int> *> *vec) { return vec->pop(); }

unsigned len_int(Vector<int> *vec) { return vec->len(); }
unsigned len_pair(Vector<Pair> *vec) { return vec->len(); }
unsigned len_str(Vector<std::string> *vec) { return vec->len(); }
unsigned len_node_int(Vector<Node<int> *> *vec) { return vec->len(); }
}
#endif
