#pragma once

#include <cstdio>
#include <cstdlib>

#include "vec.h"

enum class InputType {
   INT,
   BOOL,
   STR,
   CHAR,
};

typedef struct {
   InputType type;
   union {
      int i;
      bool b;
      char *str;
      char ch;
   };
} InputValue;

char *readInput(size_t bufsize) {
   size_t len = 0;
   char *buffer = (char *)malloc(bufsize);

   if (!buffer) {
      log(LogLevel::ERROR, "fail allocating memory");
      return NULL;
   }

   int c;
   while ((c = getchar()) != '\n' && c != EOF) {
      buffer[len++] = (char)c;

      if (len >= bufsize) {
         bufsize *= 2;
         char *tmp = (char *)realloc(buffer, bufsize);
         if (!tmp) {
            log(LogLevel::ERROR, "fail allocating memory");
            free(buffer);
            return NULL;
         }
         buffer = tmp;
      }
   }

   buffer[len] = '\0';
   return buffer;
}

InputValue readInputCastValue(InputType type, size_t bufsize) {
   InputValue val = InputValue{.type = type};

   char *input = readInput(bufsize);

   switch (type) {
   case InputType::INT:
      val.i = atoi(input);
      break;
   case InputType::BOOL:
      val.b = (atoi(input) != 0);
      break;
   case InputType::CHAR:
      val.ch = input[0];
      break;
   case InputType::STR:
      val.str = input;
      break;
   };

   if (type != InputType::STR) free(input);

   return val;
}

typedef void (*Callback)(void *ctx);
typedef struct {
   void *env;
   Callback cb;
} Closure;

typedef struct {
   char key;
   char *value;
   Closure cl;
} Pair;

class InputBuffer {
 private:
   Vector<Pair> pairs;

 public:
   InputBuffer() = default;

   InputBuffer &bind(char key, char *value, Closure cl) {
      Pair *pair = (Pair *)malloc(sizeof(Pair));
      if (!pair) {
         log(LogLevel::ERROR, "fail allocating memory");
         return *this;
      }
      pair->key = key;
      pair->value = value;
      pair->cl = cl;
      pairs.push(pair);
      return *this;
   }

   void prompt(char *str) {
      printf("%s\n", str);
      for (unsigned i = 0; i < pairs.len(); i++) {
         Pair *cur = pairs.get(i);
         if (!cur) continue;
         printf("%c - %s\n", cur->key, cur->value);
      }
   }

   void awaitInput(char *str) {
      printf("%s", str);
      InputValue input = readInputCastValue(InputType::CHAR, 1);
      if (!input.ch) {
         log(LogLevel::ERROR, "fail reading input");
         return;
      }
      for (unsigned i = 0; i < pairs.len(); i++) {
         Pair *cur = pairs.get(i);
         if (!cur) continue;
         if (cur->key != input.ch) continue;
         cur->cl.cb(cur->cl.env);
         return;
      }
      log(LogLevel::WARN, "No such option: %c", input.ch);
   }
};
