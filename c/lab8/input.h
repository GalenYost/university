#ifndef INPUT_H
#define INPUT_H

#include <stdbool.h>
#include <stddef.h>

typedef void (*Callback)(void *env);
typedef struct {
   void *env;
   Callback fn;
} Closure;

typedef enum {
   TYPE_INT,
   TYPE_STR,
   TYPE_CHAR,
   TYPE_BOOL,
   TYPE_DOUBLE,
} ValueType;

typedef struct {
   ValueType type;
   union {
      int intg;
      double dbl;
      char *str;
      char ch;
      bool bl;
   };
} Value;

typedef struct {
   char *keys;
   char **values;
   Closure *closures;
   size_t len;
   size_t cap;
} InputBuffer;

void strToLower(char *s);
void strToUpper(char *s);

InputBuffer initInputBuffer(size_t capacity);
void expandInputBuffer(InputBuffer *ib);

char *readInput(size_t bufsize);
Value readInputCastValue(ValueType type, size_t bufsize);

void attach(InputBuffer *ib, char key, char *value, Closure cl);
void prompt(InputBuffer *ib, const char *text);
void awaitInput(InputBuffer *ib);

#endif
