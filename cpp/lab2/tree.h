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

   Node() = default;
   Node(const T &v) : val(v), left(nullptr), right(nullptr) {}
};

template <typename T> void collectValues(Node<T> *node, Vector<T> *vec) {
   if (!node) return;
   vec->push(node->val);
   collectValues(node->left, vec);
   collectValues(node->right, vec);
}

int cmpInt(const void *a, const void *b) {
   int x = **(int **)a;
   int y = **(int **)b;
   return x - y;
}

template <typename T> Node<T> *buildBST(Vector<T> *vec, int l, int r) {
   if (l > r) return nullptr;
   int mid = l + (r - l) / 2;
   Node<T> *node = new Node<T>(vec->get(mid));
   node->left = buildBST(vec, l, mid - 1);
   node->right = buildBST(vec, mid + 1, r);
   return node;
}

template <typename T> struct FindCtx {
   T value;
   Node<T> *found = nullptr;
};

template <typename T> Node<T> *copySubtree(Node<T> *node) {
   if (!node) return nullptr;
   Node<T> *newNode = new Node<T>(node->val);
   newNode->left = copySubtree(node->left);
   newNode->right = copySubtree(node->right);
   return newNode;
}

template <typename T> void findCallback(void *env, Node<T> *node) {
   FindCtx<T> *ctx = (FindCtx<T> *)env;
   if (ctx->found) return;
   if (node->val == ctx->value) { ctx->found = copySubtree(node); }
}

template <typename T> class BinaryTree {
 private:
   Node<T> *head = nullptr;
   Node<T> *cur_ptr = nullptr;
   Vector<Node<T> *> path;

   BinaryTree(Node<T> *node) : head(node), cur_ptr(node) {}

   void clear(Node<T> *node) {
      if (!node) return;
      clear(node->left);
      clear(node->right);
      delete node;
   }

   Node<T> *getNthNode(Node<T> *node, int &index) const {
      if (!node) return nullptr;
      Node<T> *left = getNthNode(node->left, index);
      if (left) return left;
      if (index == 0) return node;
      index--;
      return getNthNode(node->right, index);
   }

   Node<T> *safeGetLastPath() const {
      if (path.len() == 0) return nullptr;
      Node<T> **last = path.get(path.len() - 1);
      return last ? *last : nullptr;
   }

   void displayIndented(Node<T> *node, int depth) {
      if (!node) return;
      displayIndented(node->right, depth + 1);
      for (int i = 0; i < depth; ++i) std::cout << "\t";
      if (node == cur_ptr) {
         std::cout << DEBUG_COLOR << node->val << RESET_COLOR << "\n";
      } else {
         std::cout << node->val << "\n";
      }
      displayIndented(node->left, depth + 1);
   }

   void saveNode(std::ofstream &out, Node<T> *node) {
      if (!node) {
         out << "# ";
         return;
      }
      out << node->val << " ";
      saveNode(out, node->left);
      saveNode(out, node->right);
   }

   Node<T> *loadNode(std::ifstream &in) {
      std::string token;
      if (!(in >> token)) {
         log(LogLevel::ERROR, "EOF while reading tree");
         return nullptr;
      }

      if (token == "#") return nullptr;

      T value{};
      bool ok = false;

      if constexpr (std::is_same<T, int>::value) {
         char *endptr;
         value = std::strtol(token.c_str(), &endptr, 10);
         ok = (*endptr == '\0');
      } else if constexpr (std::is_same<T, char>::value) {
         if (token.size() == 1) {
            value = token[0];
            ok = true;
         }
      } else if constexpr (std::is_same<T, bool>::value) {
         value = (token != "0");
         ok = true;
      } else {
         log(LogLevel::ERROR, "Unsupported type in loadNode");
      }

      if (!ok) {
         log(LogLevel::ERROR, "Parse error for token: " + token);
         return nullptr;
      }

      Node<T> *node = new Node<T>(value);
      node->left = loadNode(in);
      node->right = loadNode(in);
      return node;
   }

 public:
   BinaryTree() = default;

   BinaryTree(const BinaryTree &other) { head = copySubtree(other.head); }

   ~BinaryTree() {
      clear(head);
      head = nullptr;
      cur_ptr = nullptr;
   }

   const T &operator[](int n) const {
      Node<T> *result = getNthNode(head, n);
      if (!result) {
         log(LogLevel::ERROR, "BinaryTree index out of range");
         throw std::out_of_range("BinaryTree index out of range");
      }
      return result->val;
   }

   BinaryTree &operator=(const BinaryTree &other) {
      if (this == &other) return *this;
      clear(head);
      head = copySubtree(other.head);
      return *this;
   }

   BinaryTree(BinaryTree &&other) noexcept {
      head = other.head;
      cur_ptr = other.cur_ptr;
      other.head = nullptr;
      other.cur_ptr = nullptr;
   }

   BinaryTree &operator=(BinaryTree &&other) noexcept {
      if (this == &other) return *this;
      clear(head);
      head = other.head;
      cur_ptr = other.cur_ptr;
      other.head = nullptr;
      other.cur_ptr = nullptr;
      return *this;
   }

   static BinaryTree fromNode(Node<T> *node) { return BinaryTree(node); }

   void insertHead(T value, bool left = true) {
      Node<T> *new_head = new Node<T>(value);
      if (left)
         new_head->left = head;
      else
         new_head->right = head;
      head = new_head;
      cur_ptr = head;
   }

   bool insertUnderPointer(T value, Direction dir) {
      if (!cur_ptr) {
         if (!head) {
            insertHead(value, true);
            return true;
         } else {
            movePtr(Direction::HEAD);
         }
      }

      Node<T> *newNode = new Node<T>(value);

      switch (dir) {
      case Direction::LEFT:
         if (cur_ptr->left) clear(cur_ptr->left);
         cur_ptr->left = newNode;
         break;
      case Direction::RIGHT:
         if (cur_ptr->right) clear(cur_ptr->right);
         cur_ptr->right = newNode;
         break;
      default:
         log(LogLevel::WARN, "Only 'left' and 'right' are possible");
         delete newNode;
         break;
      }
      return true;
   }

   void movePtr(Direction dir) {
      if (!cur_ptr) {
         log(LogLevel::WARN, "Dropping call, no current pointer");
         return;
      }

      switch (dir) {
      case Direction::UP:
         if (path.len() > 0) {
            cur_ptr = safeGetLastPath();
            if (cur_ptr)
               path.pop();
            else
               log(LogLevel::WARN, "Path is empty");
         } else {
            log(LogLevel::WARN, "Path is empty");
         }
         break;
      case Direction::LEFT:
         if (cur_ptr->left) {
            path.push(cur_ptr);
            cur_ptr = cur_ptr->left;
         } else {
            log(LogLevel::WARN, "Left element doesn't exist");
         }
         break;
      case Direction::RIGHT:
         if (cur_ptr->right) {
            path.push(cur_ptr);
            cur_ptr = cur_ptr->right;
         } else {
            log(LogLevel::WARN, "Right element doesn't exist");
         }
         break;
      case Direction::HEAD:
         cur_ptr = head;
         path = Vector<Node<T> *>();
         break;
      }
   }

   void displayCurrent() {
      if (empty()) {
         log(LogLevel::INFO, "Tree is empty");
         return;
      }
      if (!cur_ptr) movePtr(Direction::HEAD);
      log(LogLevel::INFO,
          "Current pointer value: " + std::to_string(cur_ptr->val));
   }

   void displayTree() {
      if (empty()) {
         log(LogLevel::INFO, "Tree is empty");
         return;
      }
      log(LogLevel::INFO, "Current tree structure:");
      displayIndented(head, 0);
   }

   void saveToFile(const std::string &filename, Node<T> *node) {
      std::ofstream out(filename);
      if (!out.is_open()) {
         log(LogLevel::ERROR, "Cannot open file for writing: " + filename);
         return;
      }
      saveNode(out, node);
      out.close();
   }

   Node<T> *loadFromFile(const std::string &filename,
                         Node<T> **curPtr = nullptr) {
      std::ifstream in(filename);
      if (!in.is_open()) {
         log(LogLevel::ERROR, "Cannot open file for reading: " + filename);
         return nullptr;
      }
      Node<T> *newHead = loadNode(in);
      in.close();
      if (curPtr) *curPtr = newHead;
      return newHead;
   }

   void sortTree(Node<T> **headPtr) {
      Vector<T> values;
      collectValues(*headPtr, &values);
      qsort(values.vec, values.len(), sizeof(T), cmpInt);
      clear(*headPtr);
      *headPtr = buildBST(&values, 0, values.len() - 1);
   }

   BinaryTree<T> find(T value) {
      FindCtx<T> ctx{value, nullptr};
      traverse(head, findCallback<T>, &ctx);
      return fromNode(ctx.found);
   }

   bool empty() const { return head == nullptr; }
};
