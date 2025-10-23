#ifndef TEST_MODE

#include "input.h"
#include "log.h"
#include "tree.h"

#include <bits/stdc++.h>

void exit_fn(void *) { std::exit(0); }
void insert_fn(void *env) {
   BinaryTree<int> *bt = static_cast<BinaryTree<int> *>(env);

   std::cout << "Direction (LEFT/RIGHT/HEAD): " << std::flush;
   InputValue dir_input = readInputCastValue(InputType::STR);

   std::transform(dir_input.str.begin(), dir_input.str.end(),
                  dir_input.str.begin(), ::tolower);

   std::cout << "Value (int): " << std::flush;
   InputValue val_input = readInputCastValue(InputType::INT);

   if (dir_input.str == "left") {
      bt->insertUnderPointer(val_input.i, Direction::LEFT);
   } else if (dir_input.str == "right") {
      bt->insertUnderPointer(val_input.i, Direction::RIGHT);
   } else if (dir_input.str == "head") {
      bt->insertHead(val_input.i);
   } else {
      log(LogLevel::ERROR, "Wrong input");
      return;
   }
}
void move_fn(void *env) {
   BinaryTree<int> *bt = static_cast<BinaryTree<int> *>(env);

   std::cout << "Direction (LEFT/RIGHT/HEAD/UP): " << std::flush;
   InputValue dir_input = readInputCastValue(InputType::STR);

   std::transform(dir_input.str.begin(), dir_input.str.end(),
                  dir_input.str.begin(), ::tolower);

   Direction dir;

   if (dir_input.str == "left") {
      dir = Direction::LEFT;
   } else if (dir_input.str == "right") {
      dir = Direction::RIGHT;
   } else if (dir_input.str == "head") {
      dir = Direction::HEAD;
   } else if (dir_input.str == "up") {
      dir = Direction::UP;
   } else {
      log(LogLevel::ERROR, "Wrong input");
      return;
   }

   bt->movePtr(dir);
}

void sort_fn(void *env) {
   BinaryTree<int> *bt = static_cast<BinaryTree<int> *>(env);

   std::cout << "Order (0 - descending, 1 - ascending): " << std::flush;
   InputValue ascending = readInputCastValue(InputType::BOOL);

   if (ascending.b == 0) {
      bt->sortTree(false);
   } else {
      bt->sortTree();
   }
}

void reset_fn(void *env) {
   BinaryTree<int> *bt = static_cast<BinaryTree<int> *>(env);
   *bt = BinaryTree<int>();
}

void read_fn(void *env) {
   BinaryTree<int> *bt = static_cast<BinaryTree<int> *>(env);

   std::cout << "Filename (with extension): " << std::flush;
   InputValue fname = readInputCastValue(InputType::STR);

   bt->loadFromFile(fname.str);
}

void write_fn(void *env) {
   BinaryTree<int> *bt = static_cast<BinaryTree<int> *>(env);

   std::cout << "Filename (with extension): " << std::flush;
   InputValue fname = readInputCastValue(InputType::STR);

   bt->saveToFile(fname.str);
}

void display_fn(void *env) {
   std::cout << std::endl;
   BinaryTree<int> *bt = static_cast<BinaryTree<int> *>(env);
   bt->displayTree();
   std::cout << std::endl;
}

int main(void) {
   BinaryTree<int> bt;

   Closure exit_cl = {.cb = exit_fn};
   Closure insert_cl = {&bt, insert_fn};
   Closure move_cl = {&bt, move_fn};
   Closure sort_cl = {&bt, sort_fn};
   Closure reset_cl = {&bt, reset_fn};
   Closure read_cl = {&bt, read_fn};
   Closure write_cl = {&bt, write_fn};
   Closure display_cl = {&bt, display_fn};

   InputBuffer ib = InputBuffer();
   ib.bind('e', "exit", exit_cl)
       .bind('i', "insert", insert_cl)
       .bind('m', "move", move_cl)
       .bind('s', "sort", sort_cl)
       .bind('c', "clear", reset_cl)
       .bind('r', "read", read_cl)
       .bind('w', "write", write_cl)
       .bind('d', "display tree", display_cl);

   do {
      ib.prompt("Options:");
      ib.awaitInput("Choice: ");
   } while (true);
}

#endif
