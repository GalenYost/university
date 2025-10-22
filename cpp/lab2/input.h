#pragma once

#include <cstdlib>
#include <iostream>

#include "vec.h"

enum class InputType {
   INT,
   BOOL,
   STR,
   CHAR,
};

struct InputValue {
   InputType type;
   int i;
   bool b;
   char ch;
   std::string str;
};

std::string readInput() {
   std::string input;
   std::getline(std::cin, input);
   return input;
}

InputValue readInputCastValue(InputType type) {
   InputValue val = InputValue{.type = type};

   std::string input = readInput();

   switch (type) {
   case InputType::INT:
      val.i = std::stoi(input);
      break;
   case InputType::BOOL:
      val.b = (std::stoi(input) != 0);
      break;
   case InputType::CHAR:
      val.ch = input.empty() ? '\0' : input[0];
      break;
   case InputType::STR:
      val.str = input;
      break;
   }

   return val;
}

typedef void (*Callback)(void *ctx);
typedef struct {
   void *env;
   Callback cb;
} Closure;

typedef struct {
   char key;
   std::string value;
   Closure cl;
} Pair;

class InputBuffer {
 private:
   Vector<Pair> pairs;

 public:
   InputBuffer() = default;

   InputBuffer &bind(char key, std::string value, Closure cl) {
      Pair *pair = new Pair{key, value, cl};
      pairs.push(*pair);
      return *this;
   }

   void prompt(std::string str) {
      std::cout << str << std::endl;
      for (unsigned i = 0; i < pairs.len(); i++) {
         Pair *cur = pairs.get(i);
         if (!cur) continue;
         std::cout << cur->key << " - " << cur->value << std::endl;
      }
   }

   void awaitInput(std::string str) {
      std::cout << str << std::flush;
      InputValue input = readInputCastValue(InputType::CHAR);
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
      log(LogLevel::WARN, "No such option: " + std::to_string(input.ch));
   }
};
