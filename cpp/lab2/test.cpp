#ifdef TEST_MODE

#include <iostream>

#include "log.h"
#include "tree.h"
#include "vec.h"

inline void assert(bool condition, const std::string &msg) {
   if (condition)
      return;
   else
      log(LogLevel::ERROR, "Assetion failed: " + msg);
   std::exit(0);
}

void test_vec() {
   Vector<std::string> vec = Vector<std::string>();
   vec.push("Hello, World!");
   vec.push("Bye, World...");

   assert(*vec.get(1) == "Bye, World...",
          "first element is not 'Bye, World...'");
   assert(vec.len() == 2, "len of vec is not 2");

   vec.pop();

   assert(vec.get(1) == 0, "first element has a valid address");
   assert(vec.len() == 1, "len of vec is not 1");
}

void test_tree() {
   BinaryTree<int> bt;
   bt.insertHead(1);
   bt.insertUnderPointer(2, Direction::LEFT);
   bt.insertUnderPointer(3, Direction::RIGHT);

   assert(bt.get(0) == 2, "left isnt equal to 1");
   assert(bt.get(1) == 1, "head isnt equal to 2");
   assert(bt.get(2) == 3, "right isnt equal to 3");
}

void test_sort_tree() {
   BinaryTree<int> bt = BinaryTree<int>();
   bt.insertHead(10);
   bt.insertUnderPointer(100, Direction::LEFT);
   bt.insertUnderPointer(80, Direction::RIGHT);
   bt.movePtr(Direction::LEFT);
   bt.insertUnderPointer(70, Direction::LEFT);
   bt.insertUnderPointer(110, Direction::RIGHT);

   std::cout << "Initial tree:" << std::endl;
   bt.displayTree();

   bt.sortTree();
   std::cout << "Sorted (ascending):" << std::endl;
   bt.displayTree();

   bt.sortTree(false);
   std::cout << "Sorted (descending):" << std::endl;
   bt.displayTree();
}

int main(void) {
   test_vec();
   test_tree();
   test_sort_tree();
}

#endif
