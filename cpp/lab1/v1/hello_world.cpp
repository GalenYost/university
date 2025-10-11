#include <iostream>
#include <string>

void print_str(std::string str) {
   std::cout << str << std::endl;
}

int main(void) {
   std::string str = "Hello, World!";
   print_str(str);

   str = "Hello, World...";
   print_str(str);

   return 0;
}
