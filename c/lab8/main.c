#include "input.h"
#include "log.h"
#include "tree.h"

#include <stdlib.h>
#include <string.h>

void exit_fn(void *env) { exit(0); }
void display_fn(void *env) {
   BinaryTree *bt = env;
   printf("\n");
   display(bt);
}
void move_fn(void *env) {
   BinaryTree *bt = env;

   printf("Direction (HEAD/UP/LEFT/RIGHT): ");
   Value direction_input = readInputCastValue(TYPE_STR, 6);
   strToUpper(direction_input.str);
   printf("\n");

   Direction dir;
   if (strcmp(direction_input.str, "HEAD") == 0) {
      dir = HEAD;
   } else if (strcmp(direction_input.str, "UP") == 0) {
      dir = UP;
   } else if (strcmp(direction_input.str, "LEFT") == 0) {
      dir = LEFT;
   } else if (strcmp(direction_input.str, "RIGHT") == 0) {
      dir = RIGHT;
   } else {
      log(ERROR, "Wrong input");
      return;
   }

   movePtr(bt, dir);
}
void insert_fn(void *env) {
   BinaryTree *bt = env;
   printf("Value (int): ");
   Value val_input = readInputCastValue(TYPE_INT, 8);
   int parsed = (int)val_input.intg;
   printf("\n");

   printf("Direction (HEAD/UP/LEFT/RIGHT): ");
   Value direction_input = readInputCastValue(TYPE_STR, 6);
   printf("\n");
   Direction dir;
   if (strcmp(direction_input.str, "HEAD") == 0) {
      dir = HEAD;
   } else if (strcmp(direction_input.str, "UP") == 0) {
      dir = UP;
   } else if (strcmp(direction_input.str, "LEFT") == 0) {
      dir = LEFT;
   } else if (strcmp(direction_input.str, "RIGHT") == 0) {
      dir = RIGHT;
   } else {
      log(ERROR, "Wrong input");
      return;
   }

   insertUnderPtr(bt, dir, parsed);
}
void remove_fn(void *env) {
   BinaryTree *bt = env;

   printf("Direction (HEAD/UP/LEFT/RIGHT): ");
   Value direction_input = readInputCastValue(TYPE_STR, 6);
   Direction dir;
   printf("\n");

   if (strcmp(direction_input.str, "HEAD") == 0) {
      dir = HEAD;
   } else if (strcmp(direction_input.str, "UP") == 0) {
      dir = UP;
   } else if (strcmp(direction_input.str, "LEFT") == 0) {
      dir = LEFT;
   } else if (strcmp(direction_input.str, "RIGHT") == 0) {
      dir = RIGHT;
   } else {
      log(ERROR, "Wrong input");
      return;
   }
   removeUnderPtr(bt, dir);
}
void find_fn(void *env) {
   BinaryTree *bt = env;

   printf("Order (PREORDER/INORDER/POSTORDER): ");
   Value order = readInputCastValue(TYPE_STR, 10);
   printf("\n");
   printf("Target value (int): ");
   Value target = readInputCastValue(TYPE_INT, 8);
   printf("\n");

   SearchOrder parsed;
   if (strcmp(order.str, "PREORDER") == 0) {
      parsed = PREORDER;
   } else if (strcmp(order.str, "INORDER") == 0) {
      parsed = INORDER;
   } else if (strcmp(order.str, "POSTORDER") == 0) {
      parsed = POSTORDER;
   } else {
      log(ERROR, "Wrong input");
      return;
   }

   find(bt, parsed, target.intg);
}
void display_path_fn(void *env) {
   BinaryTree *bt = env;
   printf("\n");
   displayPath(bt);
}

int main(void) {
   BinaryTree *bt = initBinaryTree();

   InputBuffer ib = initInputBuffer(2);

   Closure exit_cl = {.fn = exit_fn, .env = NULL};
   Closure display_cl = {.fn = display_fn, .env = bt};
   Closure move_cl = {.fn = move_fn, .env = bt};
   Closure insert_cl = {.fn = insert_fn, .env = bt};
   Closure remove_cl = {.fn = remove_fn, .env = bt};
   Closure find_cl = {.fn = find_fn, .env = bt};
   Closure displayPath_cl = {.fn = display_path_fn, .env = bt};

   attach(&ib, 'e', "exit", exit_cl);
   attach(&ib, 'd', "display", display_cl);
   attach(&ib, 'm', "move pointer", move_cl);
   attach(&ib, 'i', "insert under pointer", insert_cl);
   attach(&ib, 'r', "remove under pointer", remove_cl);
   attach(&ib, 'f', "find value", find_cl);
   attach(&ib, 'p', "display path", displayPath_cl);

   do {
      prompt(&ib, "Options:");
      awaitInput(&ib);
   } while (true);
}
