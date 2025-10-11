#pragma once

#include "log.h"
#include <cstring>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

using Callback = std::function<void(void)>;

struct Pair {
   std::string key;
   std::string value;
   Callback callback;
   Pair(const std::string &k, const std::string &v, Callback cb)
       : key(k), value(v), callback(cb) {};
};

template <typename T> T readInputAndCastValue() {
   std::string input;
   std::getline(std::cin, input);

   std::istringstream iss(input);
   T value;

   if constexpr (std::is_same_v<T, std::string>) {
      return input;
   } else if (iss >> value) {
      return value;
   } else {
      throw std::runtime_error("Invalid input for given type");
   }
};

class InputBuffer {
 private:
   std::vector<Pair> pairs;

 public:
   InputBuffer() = default;

   InputBuffer &bind(std::string key, std::string value, Callback callback) {
      for (const auto &pair : pairs) {
         if (pair.key == key) return *this;
      }
      pairs.emplace_back(key, value, callback);
      return *this;
   };

   void prompt(const std::string &msg) const {
      std::cout << msg;
      if (pairs.empty()) {
         log(LogLevel::WARN, "No InputBuffer options, ignoring call");
         return;
      }
      for (const auto &pair : pairs) {
         std::cout << pair.key << " - " << pair.value << "\n";
      }
   }

   void awaitInput(const std::string &prompt) const {
      std::cout << prompt;
      std::string keyInput = readInputAndCastValue<std::string>();
      for (const auto &pair : pairs) {
         if (pair.key != keyInput) continue;
         pair.callback();
         break;
      }
   }
};
