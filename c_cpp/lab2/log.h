#pragma once

#include <iostream>
#include <sstream>

enum class LogLevel { INFO, WARN, ERROR };

inline void log(LogLevel level, const std::string &msg) {
   switch (level) {
   case LogLevel::INFO:
      std::cout << "\033[32m[INFO]\033[0m " << msg << "\n";
      break;
   case LogLevel::WARN:
      std::cout << "\033[33m[WARN]\033[0m " << msg << "\n";
      break;
   case LogLevel::ERROR:
      std::cerr << "\033[31m[ERROR]\033[0m " << msg << "\n";
      break;
   }
}
