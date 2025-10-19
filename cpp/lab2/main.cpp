#include "input.h"
#include "log.h"
#include "tree.h"

#define PROMPT_STR (char *)"Options:"
#define CHOICE_STR (char *)"Choice: "

#define EXIT_CHAR 'e'
#define EXIT_STR (char *)"exit"

void exit_fn(void *) { std::exit(0); }

// int main(void) {
//    Closure exit_cl = Closure{.cb = exit_fn};
//
//    InputBuffer ib = InputBuffer();
//    ib.bind(EXIT_CHAR, EXIT_STR, exit_cl);
//
//    do {
//       ib.prompt(PROMPT_STR);
//       ib.awaitInput(CHOICE_STR);
//    } while (true);
// }

int main(void) {
   BinaryTree<int> bt = BinaryTree<int>();
   bt.displayTree();
   bt.insertHead(1);
   bt.insertUnderPointer(2, Direction::LEFT);
   bt.insertUnderPointer(2, Direction::RIGHT);
   bt.displayTree();
   return 0;
}
