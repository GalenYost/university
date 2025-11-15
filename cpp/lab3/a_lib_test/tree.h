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

template <typename T> struct AddElementArgs {
    T val;
    Direction dir;
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
    void displayIndented(std::ostream &out, Node<T> *node, int depth) const;
    void replaceSubtree(Node<T> *&target, Node<T> *source);
    void deleteSubtree(Node<T> *node);

    Node<T> *getNthNode(Node<T> *node, unsigned &index) const;
    Node<T> *getNodeAt(unsigned depth, unsigned index) const;
    Node<T> *safeGetLastPath() const;
    Node<T> *cloneSubtree(const Node<T> *src) const;
    Node<T> *findParent(Node<T> *root, Node<T> *child) const;

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

    BinaryTree<T> &operator+(AddElementArgs<T> args);

    BinaryTree<T> &operator^(Direction dir);

    class NodeRef {
        BinaryTree<T> *tree;
        Node<T> *node;

      public:
        NodeRef(BinaryTree<T> *t, Node<T> *n);
        NodeRef &operator=(const T &new_val);
        NodeRef &operator=(const BinaryTree<T> &other);
        operator T &();
        operator const T &() const;
    };

    NodeRef operator()(unsigned depth, unsigned index);

    void sortTree(bool ascending = true);
    bool empty() const;
};
