#include <ctype.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "input.h"
#include "log.h"

void strToLower(char *s) {
   for (size_t i = 0; s[i]; i++) s[i] = tolower((unsigned char)s[i]);
}

void strToUpper(char *s) {
   for (size_t i = 0; s[i]; i++) s[i] = toupper((unsigned char)s[i]);
}

void clearStdin(void) {
   int c;
   while ((c = getchar()) != '\n' && c != EOF);
}

char *readInput(size_t bufsize) {
   char *buf = malloc(bufsize);
   if (!buf) return NULL;

   clearStdin();

   if (fgets(buf, bufsize, stdin) == NULL) {
      free(buf);
      return NULL;
   }

   buf[strcspn(buf, "\n")] = '\0';
   return buf;
}

Value readInputCastValue(ValueType type, size_t bufsize) {
   char *input = readInput(bufsize);

   Value val = {
       .type = type,
   };

   if (!input) return val;

   switch (type) {
   case TYPE_INT:
      val.intg = atoi(input);
      break;
   case TYPE_STR:
      val.str = strdup(input);
      break;
   case TYPE_CHAR:
      val.ch = (char)*input;
      break;
   case TYPE_BOOL:
      if (atoi(input) == 0 || atoi(input) == 1) { val.bl = atoi(input); }
      break;
   case TYPE_DOUBLE:
      val.dbl = atof(input);
      break;
   };

   return val;
}

InputBuffer initInputBuffer(size_t cap) {
   char *keys = (char *)malloc(cap * sizeof(char));
   char **values = (char **)malloc(cap * sizeof(char *));
   Closure *closures = (Closure *)malloc(cap * sizeof(Closure));

   if (!keys || !values || !closures) {
      log(ERROR, "failed to initialize input buffer instance");
      exit(1);
   }

   InputBuffer ib;

   ib.keys = keys;
   ib.values = values;
   ib.closures = closures;

   ib.len = 0;
   ib.cap = cap;

   return ib;
}

void expandInputBuffer(InputBuffer *ib) {
   char *new_keys = (char *)realloc(ib->keys, (ib->cap + 1) * sizeof(char));
   char **new_values =
       (char **)realloc(ib->values, (ib->cap + 1) * sizeof(char *));
   Closure *new_closures =
       (Closure *)realloc(ib->closures, (ib->cap + 1) * sizeof(Closure));

   if (!new_keys || !new_values || !new_closures) {
      log(ERROR, "failed to expand input buffer instance");
      exit(1);
   }

   ib->keys = new_keys;
   ib->values = new_values;
   ib->closures = new_closures;

   ib->cap += 1;
}

void attach(InputBuffer *ib, char key, char *value, Closure cl) {
   if (!ib || !ib->keys || !ib->values || !ib->closures) return;

   if (ib->len > 0) {
      for (size_t i = 0; i < ib->len; i++) {
         if (ib->keys[i] == key) {
            log(ERROR, "Key exists in the buffer");
            return;
         }
      }
   }

   if (ib->len >= ib->cap) { expandInputBuffer(ib); }

   ib->keys[ib->len] = key;
   ib->values[ib->len] = value;
   ib->closures[ib->len] = cl;
   ib->len++;

#ifdef DEBUG_MODE
   char buf[26 + 7];
   sprintf(buf, "new input buffer size: %lu\n", ib->len);
   log(INFO, buf);
#endif
}

void prompt(InputBuffer *ib, const char *text) {
   if (!ib || !ib->keys || !ib->values || !ib->closures) return;
   printf("\n%s\n", text);

   if (ib->len > 0) {
      for (size_t i = 0; i < ib->len; i++) {
         printf("%c - %s\n", ib->keys[i], ib->values[i]);
      }
   } else {
      log(WARN, "Buffer is empty");
      return;
   }
}

void awaitInput(InputBuffer *ib) {
   printf("Choice: ");
   Value key = readInputCastValue(TYPE_CHAR, 1);

   if (!key.ch) {
      log(ERROR, "Wrong input");
      return;
   }

   if (ib->len > 0) {
      for (size_t i = 0; i < ib->len; i++) {
         if (ib->keys[i] == key.ch) { ib->closures[i].fn(ib->closures[i].env); }
      }
   } else {
      log(WARN, "Buffer is empty");
      return;
   }
}
