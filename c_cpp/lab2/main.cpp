#include "tree.h"

int main(void) {
   BinaryTree<int> bt(0);

   // --------------- left side ------------------ \\

   bt.insertUnderPointer(1, true);
   bt.insertUnderPointer(2, false);

   bt.movePtrToNext(true); // moving down to left side

   bt.insertUnderPointer(3, true);
   bt.insertUnderPointer(4, false);

   bt.movePtrToNext(true); // moving down to left side

   bt.insertUnderPointer(7, true);
   bt.insertUnderPointer(8, false);

   bt.movePtrToHead(); // moving back

   bt.movePtrToNext(true);  // moving down to left side
   bt.movePtrToNext(false); // moving down to right side

   bt.insertUnderPointer(9, true);
   bt.insertUnderPointer(10, false);

   // --------------- right side ------------------ \\

   bt.movePtrToHead(); // moving back

   bt.movePtrToNext(false); // moving down to right side

   bt.insertUnderPointer(5, true);
   bt.insertUnderPointer(6, false);

   bt.movePtrToHead(); // moving back

   bt.movePtrToNext(false); // moving down to left side
   bt.movePtrToNext(true);  // moving down to right side

   bt.insertUnderPointer(11, true);
   bt.insertUnderPointer(12, false);

   bt.movePtrToHead(); // moving back

   bt.movePtrToNext(false); // moving down to left side
   bt.movePtrToNext(false); // moving down to right side

   bt.insertUnderPointer(13, true);
   bt.insertUnderPointer(14, false);

   bt.displayTree();
}
