#pragma once

#include "log.h"

template <typename T> class Vector {
 private:
   T *vec;
   unsigned capacity;
   unsigned count;

   void clear() {
      delete[] vec;
      capacity = 0;
      count = 0;
   }

 public:
   Vector() : vec(nullptr), capacity(0), count(0) {}
   ~Vector() { clear(); }

   void push(const T &el) {
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
   void push(const T &el, unsigned idx) {
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

   T pop() {
      T el = vec[--count];
      return el;
   }

   T *get(unsigned idx) const {
      if (idx >= count) return nullptr;
      return &vec[idx];
   }

   unsigned len() const { return count; }
   unsigned cap() const { return capacity; }
};
