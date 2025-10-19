#pragma once

#include <cstdarg>
#include <cstdio>

#define DEBUG_COLOR "\033[36m"
#define INFO_COLOR "\033[32m"
#define WARN_COLOR "\033[33m"
#define ERROR_COLOR "\033[31m"
#define RESET_COLOR "\033[0m"

enum class LogLevel {
   INFO,
   WARN,
   ERROR,
};

inline void log(LogLevel level, const char *__restrict format, ...) {
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

   printf("%s%s%s ", color, prefix, RESET_COLOR);

   va_list args;
   va_start(args, format);
   vprintf(format, args);
   va_end(args);

   printf("\n");
}
