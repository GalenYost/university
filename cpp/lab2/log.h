#pragma once

#include <iostream>
#include <sstream>

#define DEBUG_COLOR "\033[36m"
#define INFO_COLOR "\033[32m"
#define WARN_COLOR "\033[33m"
#define ERROR_COLOR "\033[31m"
#define RESET_COLOR "\033[0m"

enum class LogLevel {
   INFO,
   WARN,
   ERROR,
#ifdef DEBUG_MODE
   DEBUG
#endif
};

inline void log(LogLevel level, const std::string &msg) {
   switch (level) {
   case LogLevel::INFO:
      std::cout << INFO_COLOR << "[INFO]" << RESET_COLOR << " " << msg
                << std::endl;
      break;
   case LogLevel::WARN:
      std::cout << WARN_COLOR << "[WARN]" << RESET_COLOR << " " << msg
                << std::endl;
      break;
   case LogLevel::ERROR:
      std::cerr << ERROR_COLOR << "[ERROR]" << RESET_COLOR << " " << msg
                << std::endl;
      break;
#ifdef DEBUG_MODE
   case LogLevel::DEBUG:
      std::cout << DEBUG_COLOR << "[DEBUG]" << RESET_COLOR << " " << msg
                << std::endl;
#endif
   }
}
