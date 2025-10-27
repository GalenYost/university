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

   Node<T> *node = new Node<T>(*vec->get(mid));
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

template <typename T> void quicksort(Vector<T> *vec, int l, int r) {
   if (l >= r) return;

   T pivot = *vec->get(r);
   int i = l - 1;

   for (int j = l; j < r; ++j) {
      if (*vec->get(j) <= pivot) {
         ++i;
         T temp = *vec->get(i);
         *vec->get(i) = *vec->get(j);

         *vec->get(j) = temp;
      }
   }
   T temp = *vec->get(i + 1);
   *vec->get(i + 1) = *vec->get(r);
   *vec->get(r) = temp;

   quicksort(vec, l, i);
   quicksort(vec, i + 2, r);
}

template <typename T> class BinaryTree {
 private:
   Node<T> *head = nullptr;
   Node<T> *cur_ptr = nullptr;
   Vector<Node<T> *> path = Vector<Node<T> *>();

   void clear(Node<T> *node) {
      if (!node) return;
      clear(node->left);
      clear(node->right);
      delete node;
   }

   Node<T> *getNthNode(Node<T> *node, unsigned &index) const {
      if (!node) return nullptr;

      Node<T> *found = getNthNode(node->left, index);
      if (found) return found;

      if (index == 0) return node;
      index--;

      return getNthNode(node->right, index);
   }

   Node<T> *safeGetLastPath() const {
      if (path.len() == 0) return nullptr;
      Node<T> **last = path.get(path.len() - 1);
      return last ? *last : nullptr;
   }

   void replaceSubtree(Node<T> *&ptr, Node<T> *newNode) {
      if (ptr) { clear(ptr); }
      ptr = newNode;
   }

   void displayIndented(std::ostream &out, Node<T> *node, int depth) const {
      if (!node) return;
      displayIndented(out, node->right, depth + 1);
      for (int i = 0; i < depth; ++i) out << "\t";
      if (node == cur_ptr) {
         out << DEBUG_COLOR << node->val << RESET_COLOR << "\n";
      } else {
         out << node->val << "\n";
      }
      displayIndented(out, node->left, depth + 1);
   }

   void saveNode(std::ostream &out, Node<T> *node) const {
      if (!node) {
         out << "# ";
         return;
      }
      out << node->val << " ";
      saveNode(out, node->left);
      saveNode(out, node->right);
   }

   Node<T> *loadNode(std::istream &in) const {
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
   BinaryTree(Node<T> *node) : head(node), cur_ptr(node) {}

   ~BinaryTree() {
      clear(head);
      head = nullptr;
      cur_ptr = nullptr;
   }

   const T &operator[](unsigned n) const {
      Node<T> *result = getNthNode(head, n);
      if (!result) {
         log(LogLevel::ERROR, "BinaryTree index out of range");
         std::exit(0);
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

   friend std::ostream &operator<<(std::ostream &out,
                                   const BinaryTree<T> &tree) {
      tree.displayIndented(out, tree.head, 0);
      return out;
   }

   friend std::istream &operator>>(std::istream &in, BinaryTree<T> &tree) {
      tree.head = tree.loadNode(in);
      tree.cur_ptr = tree.head;
      return in;
   }

   friend const BinaryTree<T> &operator<<(const std::string &filename,
                                          const BinaryTree<T> &tree) {
      std::ofstream out(filename);
      if (!out.is_open()) {
         log(LogLevel::ERROR, "Cannot open file for writing: " + filename);
         return tree;
      }
      tree.saveNode(out, tree.head);
      return tree;
   }

   friend const BinaryTree<T> &operator>>(const std::string &filename,
                                          BinaryTree<T> &tree) {
      std::ifstream in(filename);
      if (!in.is_open()) {
         log(LogLevel::ERROR, "Cannot open file for reading: " + filename);
         return tree;
      }
      tree.head = tree.loadNode(in);
      tree.cur_ptr = tree.head;
      return tree;
   }

   BinaryTree<T> &operator+(std::pair<T, Direction> p) {
      if (!cur_ptr) { *this ^ Direction::HEAD; }

      Node<T> *newNode = new Node<T>(p.first);

      switch (p.second) {
      case Direction::LEFT:
         replaceSubtree(cur_ptr->left, newNode);
         break;
      case Direction::RIGHT:
         replaceSubtree(cur_ptr->right, newNode);
         break;
      case Direction::HEAD: {
         Node<T> *new_head = new Node<T>(p.first);
         new_head->left = head;
         head = new_head;
         cur_ptr = head;
         break;
      }
      default:
         log(LogLevel::WARN, "Only 'left', 'right' and 'head' are possible");
         delete newNode;
         break;
      }
      return *this;
   }
   BinaryTree<T> &operator+(std::pair<T, std::string> p) {
      if (!cur_ptr) { *this ^ Direction::HEAD; }

      Node<T> *newNode = new Node<T>(p.first);

      if (p.second == "left") {
         replaceSubtree(cur_ptr->left, newNode);
      } else if (p.second == "right") {
         replaceSubtree(cur_ptr->right, newNode);
      } else if (p.second == "head") {
         Node<T> *new_head = new Node<T>(p.first);
         new_head->left = head;
         head = new_head;
         cur_ptr = head;
      } else {
         log(LogLevel::WARN, "Only 'left', 'right' and 'head' are possible");
         delete newNode;
      }
      return *this;
   }

   BinaryTree<T> &operator^(Direction dir) {
      if (!cur_ptr) {
         log(LogLevel::WARN, "Dropping call, no current pointer");
         return *this;
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

      return *this;
   }
   BinaryTree<T> &operator^(const std::string &dir) {
      if (!cur_ptr) {
         log(LogLevel::WARN, "Dropping call, no current pointer");
         return *this;
      }

      if (dir == "up") {
         if (path.len() > 0) {
            cur_ptr = safeGetLastPath();
            if (cur_ptr)
               path.pop();
            else
               log(LogLevel::WARN, "Path is empty");
         } else {
            log(LogLevel::WARN, "Path is empty");
         }
      } else if (dir == "left") {
         if (cur_ptr->left) {
            path.push(cur_ptr);
            cur_ptr = cur_ptr->left;
         } else {
            log(LogLevel::WARN, "Left element doesn't exist");
         }
      } else if (dir == "right") {
         if (cur_ptr->right) {
            path.push(cur_ptr);
            cur_ptr = cur_ptr->right;
         } else {
            log(LogLevel::WARN, "Right element doesn't exist");
         }
      } else {
         cur_ptr = head;
         path = Vector<Node<T> *>();
      }

      return *this;
   }

   BinaryTree<T> find(T value) {
      FindCtx<T> ctx{value, nullptr};
      traverse(head, findCallback<T>, &ctx);
      return fromNode(ctx.found);
   }

   void sortTree(bool ascending = true) {
      if (!head) return;

      Vector<T> values;
      collectValues(head, &values);

      if (values.len() == 0) return;

      quicksort(&values, 0, values.len() - 1);

      if (!ascending) {
         for (unsigned i = 0, j = values.len() - 1; i < j; ++i, --j) {
            T temp = *values.get(i);
            *values.get(i) = *values.get(j);
            *values.get(j) = temp;
         }
      }

      Node<T> *newHead = buildBST(&values, 0, values.len() - 1);

      clear(head);
      head = newHead;
      cur_ptr = head;
      path = Vector<Node<T> *>();
   }

   bool empty() const { return head == nullptr; }
};
