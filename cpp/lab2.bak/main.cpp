#include "input.h"
#include "tree.h"

#include <functional>
#include <iostream>

int main(void) {
   BinaryTree<int> bt;

   Callback exit = []() { std::exit(0); };
   Callback move = [&bt]() {
      std::cout << "Direction (up/left/right/head): ";
      const std::string direction_str = readInputAndCastValue<std::string>();
      std::cout << std::endl;

      const Direction dir_parsed = parse_direction(direction_str);
      bt.movePtr(dir_parsed);
      std::cout << std::endl;
   };
   Callback insert = [&bt]() {
      std::cout << "Value: ";
      const int value = readInputAndCastValue<int>();

      if (bt.empty()) {
         bt.insertHead(value);
      } else {
         std::cout << "Direction (left/right): ";
         const std::string direction_str = readInputAndCastValue<std::string>();

         std::cout << std::endl;
         const Direction dir_parsed = parse_direction(direction_str);
         bt.insertUnderPointer(value, dir_parsed);
      }

      std::cout << std::endl;
   };
   Callback save = [&bt]() {
      std::cout << "File name (with extension, e.g file.txt): ";
      const std::string fname = readInputAndCastValue<std::string>();

      std::cout << std::endl;
      bt.saveToFile(fname);
      std::cout << std::endl;
   };
   Callback load = [&bt]() {
      std::cout << "File name (with extension, e.g file.txt): ";
      const std::string fname = readInputAndCastValue<std::string>();

      std::cout << std::endl;
      bt.loadFromFile(fname);
      std::cout << std::endl;
   };
   Callback sort = [&bt]() {
      std::cout << "Sorting order (ascending: 1 or 0): ";
      const bool ascending = readInputAndCastValue<bool>();

      if (ascending)
         bt.sort(std::less());
      else
         bt.sort(std::greater());
      std::cout << std::endl;
   };
   Callback find = [&bt]() {
      std::cout << "Value to search for: ";
      const int value = readInputAndCastValue<int>();
      std::cout << std::endl;

      BinaryTree<int> found = bt.find(value);
      found.displayTree();

      std::cout << std::endl;
   };
   Callback display = [&bt]() {
      std::cout << std::endl;
      bt.displayTree();
      std::cout << std::endl;
   };
   Callback display_ptr = [&bt]() {
      std::cout << std::endl;
      bt.displayCurrent();
      std::cout << std::endl;
   };

   InputBuffer ib;
   ib.bind("0", "exit", exit)
       .bind("1", "insert value", insert)
       .bind("2", "move pointer", move)
       .bind("3", "sort", sort)
       .bind("4", "find subtree", find)
       .bind("w", "write to file", save)
       .bind("r", "read from file", load)
       .bind("pt", "print whole tree", display)
       .bind("pp", "print value under the pointer", display_ptr);

   while (true) {
      ib.prompt("Options:\n");
      ib.awaitInput("Choice: ");
   }
}
