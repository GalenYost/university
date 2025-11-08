#pragma once

#include "input.h"
#include "vec.h"
#include <fstream>
#include <iostream>
#include <string>

enum class Direction { UP, LEFT, RIGHT, HEAD };

template <typename T> struct Node {
    T val;
    Node *left = nullptr;
    Node *right = nullptr;

    Node();
    Node(const T &v);
};

inline int cmpInt(const void *a, const void *b);

template <typename T> void collectValues(Node<T> *node, Vector<T> *vec);

template <typename T> Node<T> *buildBST(Vector<T> *vec, int l, int r);
template <typename T> Node<T> *copySubtree(Node<T> *node);

template <typename T> void quicksort(Vector<T> *vec, int l, int r);

template <typename T> class BinaryTree;

template <typename T>
std::ostream &operator<<(std::ostream &, const BinaryTree<T> &);

template <typename T> std::istream &operator>>(std::istream &, BinaryTree<T> &);

template <typename T> class BinaryTree {
  private:
    Node<T> *head = nullptr;
    Node<T> *cur_ptr = nullptr;
    Vector<Node<T> *> path = Vector<Node<T> *>();

    void clear(Node<T> *node);
    void replaceSubtree(Node<T> *&ptr, Node<T> *newNode);
    void displayIndented(std::ostream &out, Node<T> *node, int depth) const;

    Node<T> *getNthNode(Node<T> *node, unsigned &index) const;
    Node<T> *safeGetLastPath() const;

    void saveNode(std::ostream &out, Node<T> *node) const;
    Node<T> *loadNode(std::istream &in) const;

  public:
    BinaryTree();
    BinaryTree(const BinaryTree &other);
    BinaryTree(Node<T> *node);
    BinaryTree(BinaryTree &&other) noexcept;

    ~BinaryTree();

    const T &operator[](unsigned n) const;

    BinaryTree &operator=(const BinaryTree &other);
    BinaryTree &operator=(BinaryTree &&other) noexcept;

    friend std::ostream &operator<< <T>(std::ostream &, const BinaryTree<T> &);
    friend std::istream &operator>> <T>(std::istream &, BinaryTree<T> &);

    BinaryTree<T> &operator+(std::pair<T, Direction> p);
    BinaryTree<T> &operator+(std::pair<T, std::string> p);

    BinaryTree<T> &operator^(Direction dir);
    BinaryTree<T> &operator^(const std::string &dir);

    void save(const std::string &filename) const;
    void load(const std::string &filename);

    void sortTree(bool ascending = true);

    bool empty() const;
};
