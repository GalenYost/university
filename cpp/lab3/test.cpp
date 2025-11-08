#ifdef TEST_MODE

#include <iostream>

#include "log.h"
#include "tree.h"
#include "vec.h"
#include "wrapper.h"

inline void assert(bool condition, const std::string &msg) {
    if (condition)
        return;
    else
        log(LogLevel::ERROR, "Assertion failed: " + msg);
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
    bt + std::make_pair(1, Direction::HEAD);
    bt + std::make_pair(2, Direction::LEFT);
    bt + std::make_pair(3, Direction::RIGHT);

    assert(bt[0] == 2, "left isnt equal to 1");
    assert(bt[1] == 1, "head isnt equal to 2");
    assert(bt[2] == 3, "right isnt equal to 3");
}

void test_sort_tree() {
    BinaryTree<int> bt = BinaryTree<int>();
    bt + std::make_pair(10, Direction::HEAD);
    bt + std::make_pair(100, Direction::LEFT);
    bt + std::make_pair(80, Direction::RIGHT);
    bt ^ Direction::LEFT;
    bt + std::make_pair(70, Direction::LEFT);
    bt + std::make_pair(110, Direction::RIGHT);
    bt ^ Direction::HEAD;

    assert(bt[0] == 70, "left isnt equal to 70");

    bt.sortTree();
    assert(bt[0] == 10, "left isnt equal to 10");

    bt.sortTree(false);
    assert(bt[0] == 110, "left isnt equal to 110");
}

void test_access_wrapper() {
    BinaryTree<int> bt = BinaryTree<int>();
    bt + std::make_pair(10, Direction::HEAD);

    AccessWrapper<BinaryTree<int>> wrapper =
        AccessWrapper<BinaryTree<int>>(&bt);

    assert((*wrapper)[0] == 10, "head isnt 10");
}

int main(void) {
    test_vec();
    log(LogLevel::INFO, "Vector tests passed");
    test_tree();
    log(LogLevel::INFO, "Tree tests passed");
    test_sort_tree();
    log(LogLevel::INFO, "Tree sorting tests passed");
    test_access_wrapper();
    log(LogLevel::INFO, "Access wrapper tests passed");
}

#endif
