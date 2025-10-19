#pragma once

#include "log.h"
#include <cstdlib>

template <typename E> struct DynArray {
   E *data;
   size_t len;
   size_t cap;

   DynArray() : data(nullptr), len(0), cap(0) {}
   ~DynArray() { free(data); }

   void push(E x) {
      if (len >= cap) {
         size_t newcap = cap ? cap * 2 : 4;
         E *tmp = (E *)realloc(data, newcap * sizeof(E));
         if (!tmp) {
            log(LogLevel::ERROR, "DynArray realloc failed");
            return;
         }
         data = tmp;
         cap = newcap;
      }
      data[len++] = x;
   }

   void clear() {
      free(data);
      data = nullptr;
      len = cap = 0;
   }
};
