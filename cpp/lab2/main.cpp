#include "input.h"
#include "log.h"
#include "tree.h"

void exit_fn(void *) { std::exit(0); }

int main(void) {
   Closure exit_cl = Closure{.cb = exit_fn};

   InputBuffer ib = InputBuffer();
   ib.bind('e', "exit", exit_cl);
   ib.bind('e', "exit", exit_cl);

   do {
      ib.prompt("Options:");
      ib.awaitInput("Choice: ");
   } while (true);
}
