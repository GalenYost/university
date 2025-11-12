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

    AccessWrapper<BinaryTree<int>> wrapper(&bt);
    assert((*wrapper)[0] == 10, "head isnt 10");
}

void test_bt_input() {
    BinaryTree<int> bt = BinaryTree<int>();
    std::cout << "Input BinaryTree structure (inline): ";
    std::cin >> bt;
    std::cout << bt;

    assert(!bt.empty(), "tree is empty");
}

void test_bt_pseudo() {
    BinaryTree<int> bt;
    bt + std::make_pair(10, Direction::HEAD);
    bt + std::make_pair(9, Direction::LEFT);
    bt + std::make_pair(8, Direction::RIGHT);
    bt ^ "left";

    bt + std::make_pair(11, Direction::LEFT);

    assert(bt(2, 0) == 11, "first element on depth 2 is not 11");

    bt(2, 0) = 12;

    assert(bt(2, 0) == 12, "first element on depth 2 is not 12");

    BinaryTree<int> bt2;
    bt2 + std::make_pair(15, Direction::HEAD);
    bt2 + std::make_pair(16, Direction::LEFT);

    bt(2, 0) = bt2;

    std::cout << bt;

    assert(bt(2, 0) == 15, "first element on depth 2 is not 15");
    assert(bt(3, 0) == 16, "first element on depth 3 is not 16");
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
    test_bt_input();
    log(LogLevel::INFO, "Tree input tests passed");
    test_bt_pseudo();
    log(LogLevel::INFO, "Tree pseudo-variable tests passed");
}

#endif
