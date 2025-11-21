#include "tree.h"
#include "wrapper.h"
#include <iostream>

int main(void) {
    BinaryTree<int> bt;
    AccessWrapper<BinaryTree<int>> wrapper(&bt);

    AddElementArgs<int> head =
        AddElementArgs<int>{.val = 1, .dir = Direction::HEAD};
    (*wrapper) + head;
    AddElementArgs<int> left =
        AddElementArgs<int>{.val = 2, .dir = Direction::LEFT};
    (*wrapper) + left;

    (*wrapper) ^ Direction::LEFT;

    AddElementArgs<int> left_right =
        AddElementArgs<int>{.val = 3, .dir = Direction::RIGHT};
    (*wrapper) + left_right;

    std::cout << "Count: " << wrapper.current_count() << std::endl;

    std::cout << *wrapper;
}
