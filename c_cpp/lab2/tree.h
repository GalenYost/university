#pragma once

#include <cstdio>
#include <functional>

#include "log.h"

template <typename T> struct Node {
   T val;
   Node *left;
   Node *right;
};

template <typename T> class BinaryTree {
 private:
   Node<T> *head = nullptr;
   Node<T> *cur_ptr = nullptr;

   void traverse(Node<T> *node, std::function<void(Node<T> *)> callback) {
      if (!node)
         return;

      traverse(node->left, callback);
      callback(node);
      traverse(node->right, callback);
   };

   void displayIndented(Node<T> *node, int depth) {
      if (!node)
         return;
      displayIndented(node->right, depth + 1);
      for (int i = 0; i < depth; ++i)
         std::cout << "    ";
      std::cout << node->val << "\n";
      displayIndented(node->left, depth + 1);
   }

 public:
   BinaryTree<T>(T initial_value) {
      head = new Node<T>{initial_value, nullptr, nullptr};
      cur_ptr = head;
   };
   ~BinaryTree() { clear(head); };

   void clear(Node<T> *node) {
      if (!node)
         return;
      clear(node->left);
      clear(node->right);
      delete node;
      cur_ptr = nullptr;
   };

   void insertHead(T value, bool left) {
      Node<T> *new_head = new Node<T>{value};
      if (left) {
         new_head->left = head;
      } else {
         new_head->right = head;
      }
      head = new_head;
      cur_ptr = head;
   }

   void insertUnderPointer(T value, bool left) {
      if (!cur_ptr) {
         if (!head) {
            return;
         } else {
            movePtrToHead();
         }
      }

      if (left && !cur_ptr->left) {
         cur_ptr->left = new Node<T>{value, nullptr, nullptr};
      } else if (!left && !cur_ptr->right) {
         cur_ptr->right = new Node<T>{value, nullptr, nullptr};
      }
   }

   void movePtrToHead() { cur_ptr = head; };
   void movePtrToNext(bool left) {
      if (!cur_ptr) {
         return;
      }

      if (left)
         cur_ptr = cur_ptr->left;
      else
         cur_ptr = cur_ptr->right;
   };

   void iter(std::function<void(Node<T> *)> callback) {
      traverse(head, callback);
   };

   void displayCurrent() const {
      if (!cur_ptr) {
         std::cout << nullptr << std::endl;
         return;
      }
      std::cout << cur_ptr->val << std::endl;
   };
   void displayTree() { displayIndented(head, 0); };

   void sort(); // todo: args

   Node<T> *find(T value) const;
};
