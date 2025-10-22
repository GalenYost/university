#pragma once

#include <iostream>

constexpr const char *DEBUG_COLOR = "\033[36m";
constexpr const char *INFO_COLOR = "\033[32m";
constexpr const char *WARN_COLOR = "\033[33m";
constexpr const char *ERROR_COLOR = "\033[31m";
constexpr const char *RESET_COLOR = "\033[0m";

enum class LogLevel {
   INFO,
   WARN,
   ERROR,
};

inline void log(LogLevel level, const std::string &msg) {
   const char *color;
   const char *prefix;

   switch (level) {
   case LogLevel::INFO:
      color = INFO_COLOR;
      prefix = "[INFO]";
      break;
   case LogLevel::WARN:
      color = WARN_COLOR;
      prefix = "[WARN]";
      break;
   case LogLevel::ERROR:
      color = ERROR_COLOR;
      prefix = "[ERROR]";
      break;
   default:
      color = RESET_COLOR;
      prefix = "[LOG]";
      break;
   }

   std::cout << color << prefix << RESET_COLOR << " " << msg << std::endl;
}
