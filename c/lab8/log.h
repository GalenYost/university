#ifndef LOG_H
#define LOG_H

#include <stdio.h>

#define DEBUG_COLOR "\033[36m"
#define INFO_COLOR "\033[32m"
#define WARN_COLOR "\033[33m"
#define ERROR_COLOR "\033[31m"
#define RESET_COLOR "\033[0m"

typedef enum {
   INFO,
   WARN,
   ERROR,
} LogLevel;

static inline void log(LogLevel level, char *msg) {
   switch (level) {
   case INFO:
      printf("%s[INFO]%s %s\n", INFO_COLOR, RESET_COLOR, msg);
      break;
   case WARN:
      printf("%s[WARN]%s %s\n", WARN_COLOR, RESET_COLOR, msg);
      break;
   case ERROR:
      printf("%s[ERROR]%s %s\n", ERROR_COLOR, RESET_COLOR, msg);
      break;
   }
}

#endif
