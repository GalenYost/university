#pragma once

#include <algorithm>
#include <fstream>
#include <functional>
#include <string>
#include <vector>

#include "log.h"

enum Direction {
   UP,
   LEFT,
   RIGHT,
   HEAD,
};

static const std::unordered_map<std::string, Direction> str_to_dir{
    {"up", Direction::UP},
    {"left", Direction::LEFT},
    {"right", Direction::RIGHT},
    {"head", Direction::HEAD}};

inline Direction parse_direction(const std::string &str) {
   auto it = str_to_dir.find(str);
   if (it != str_to_dir.end()) {
      Direction dir_parsed = it->second;
      return dir_parsed;
   } else {
      log(LogLevel::ERROR, "Wrong direction input, returning to head");
      return Direction::HEAD;
   }
}

template <typename T> struct Node {
   T val;
   Node *left;
   Node *right;
};

template <typename T> class BinaryTree {
 private:
   Node<T> *head = nullptr;
   Node<T> *cur_ptr = nullptr;
   std::vector<Node<T> *> path;

   BinaryTree(Node<T> *node) : head(node), cur_ptr(node) {}

   void clear(Node<T> *node) {
      if (!node) return;
      clear(node->left);
      clear(node->right);
      delete node;
   };

   Node<T> *copySubtree(Node<T> *node) {
      if (!node) return nullptr;
      Node<T> *newNode = new Node<T>{node->val};
      newNode->left = copySubtree(node->left);
      newNode->right = copySubtree(node->right);
      return newNode;
   }

   Node<T> *getNthNode(Node<T> *node, int &index) const {
      if (!node) return nullptr;

      Node<T> *left = getNthNode(node->left, index);
      if (left) return left;

      if (index == 0) return node;
      index--;

      return getNthNode(node->right, index);
   }

   void traverse(Node<T> *node, std::function<void(Node<T> *)> callback) {
      if (!node) return;

      traverse(node->left, callback);
      callback(node);
      traverse(node->right, callback);
   };

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

   void saveNode(std::ostream &out, Node<T> *node) {
      if (!node) {
         out << "# ";
         return;
      }
      out << node->val << " ";
      saveNode(out, node->left);
      saveNode(out, node->right);
   }

   Node<T> *loadNode(std::istream &in) {
      std::string token;
      if (!(in >> token)) return nullptr;

      if (token == "#") return nullptr;

      T value;
      std::istringstream iss(token);
      iss >> value;

      Node<T> *node = new Node<T>{value, nullptr, nullptr};
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
      path.clear();
   };

   const T &operator[](int n) const {
      int idx = n;
      Node<T> *result = getNthNode(head, idx);
      if (!result) throw std::out_of_range("BinaryTree index out of range");
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
      other.head = nullptr;
   }

   BinaryTree &operator=(BinaryTree &&other) noexcept {
      if (this == &other) return *this;
      clear(head);
      head = other.head;
      other.head = nullptr;
      return *this;
   }

   static BinaryTree fromNode(Node<T> *node) { return BinaryTree(node); }

   static BinaryTree fromValue(T value) {
      Node<T> node = new Node<T>{value, nullptr, nullptr};
      return BinaryTree(node);
   }

   void insertHead(T value, bool left = true) {
      Node<T> *new_head = new Node<T>{value, nullptr, nullptr};
      if (left) {
         new_head->left = head;
      } else {
         new_head->right = head;
      }
      head = new_head;
      cur_ptr = head;
   }

   bool insertUnderPointer(T value, Direction dir) {
      if (!cur_ptr) {
         if (!head) {
            insertHead(value, true);
#ifdef DEBUG_MODE
            log(LogLevel::DEBUG, "New head assigned");
#endif
            return true;
         } else {
            movePtr(Direction::HEAD);
         }
      }

      switch (dir) {
      case LEFT:
         if (cur_ptr->left) clear(cur_ptr->left);
         cur_ptr->left = new Node<T>{value, nullptr, nullptr};
#ifdef DEBUG_MODE
         log(LogLevel::DEBUG,
             "New value for left pointer assigned: " + std::to_string(value));
#endif
         break;
      case RIGHT:
         if (cur_ptr->right) clear(cur_ptr->right);
         cur_ptr->right = new Node<T>{value, nullptr, nullptr};
#ifdef DEBUG_MODE
         log(LogLevel::DEBUG,
             "New value for right pointer assigned: " + std::to_string(value));
#endif
         break;

      default:
         log(LogLevel::WARN, "Only 'left' and 'right' are possible");
         break;
      };

      return true;
   }

   void movePtr(Direction dir) {
      if (!cur_ptr) {
         log(LogLevel::WARN, "Dropping call, no current pointer");
         return;
      }

      switch (dir) {
      case UP:
         if (!path.empty()) {
            cur_ptr = path.back();
            path.pop_back();
#ifdef DEBUG_MODE
            log(LogLevel::DEBUG, "Moving up");
#endif
         } else {
            log(LogLevel::WARN, "Dropping call, path is empty");
         }
         break;

      case LEFT:
         if (cur_ptr->left) {
            path.push_back(cur_ptr);
            cur_ptr = cur_ptr->left;
#ifdef DEBUG_MODE
            log(LogLevel::DEBUG, "Moving left");
#endif
         } else {
            log(LogLevel::WARN, "Left element doesnt exist");
         }
         break;

      case RIGHT:
         if (cur_ptr->right) {
            path.push_back(cur_ptr);
            cur_ptr = cur_ptr->right;
#ifdef DEBUG_MODE
            log(LogLevel::DEBUG, "Moving right");
#endif
         } else {
            log(LogLevel::WARN, "Right element doesnt exist");
         }
         break;

      case HEAD:
         cur_ptr = head;
         path.clear();
#ifdef DEBUG_MODE
         log(LogLevel::DEBUG, "Moving to head");
#endif
         break;
      };

      return;
   }

   void iter(std::function<void(Node<T> *)> callback) {
      traverse(head, callback);
   }

   void displayCurrent() {
      if (this->empty()) {
         log(LogLevel::INFO, "Tree is empty");
         return;
      }
      if (!cur_ptr) {
#ifdef DEBUG_MODE
         log(LogLevel::DEBUG, "Assigning cur_ptr to head");
#endif
         movePtr(Direction::HEAD);
      }
      log(LogLevel::INFO,
          "Current pointer value: " + std::to_string(cur_ptr->val));
   };
   void displayTree() {
      if (this->empty()) {
         log(LogLevel::INFO, "Tree is empty");
         return;
      }
      log(LogLevel::INFO, "Current tree structure:");
      displayIndented(head, 0);
   };

   void saveToFile(const std::string &filename) {
      std::ofstream ofs(filename);
      if (!ofs) {
         log(LogLevel::ERROR, "Cannot open file for writing: " + filename);
         return;
      }
      saveNode(ofs, head);
#ifdef DEBUG_MODE
      log(LogLevel::DEBUG, "Saved tree to file: " + filename);
#endif
   }

   void loadFromFile(const std::string &filename) {
      std::ifstream ifs(filename);
      if (!ifs) {
         log(LogLevel::ERROR, "Cannot open file for reading: " + filename);
         return;
      }

      clear(head);
      head = loadNode(ifs);
      cur_ptr = head;
      path.clear();

#ifdef DEBUG_MODE
      log(LogLevel::DEBUG, "Loaded new tree from file: " + filename);
#endif
   }

   bool empty() const { return head == nullptr; }

   void sort(std::function<bool(const T &, const T &)> comp = std::less<T>()) {
      std::vector<T> values;
      iter([&](Node<T> *node) { values.push_back(node->val); });

      std::sort(values.begin(), values.end(), comp);
      clear(head);

      std::function<Node<T> *(int, int)> buildBST = [&](int l,
                                                        int r) -> Node<T> * {
         if (l > r) return nullptr;
         int mid = l + (r - l) / 2;
         Node<T> *node = new Node<T>{values[mid], nullptr, nullptr};
         node->left = buildBST(l, mid - 1);
         node->right = buildBST(mid + 1, r);
         return node;
      };

      head = buildBST(0, values.size() - 1);
      cur_ptr = head;
      path.clear();
   };

   BinaryTree<T> find(T value) {
      Node<T> *found = nullptr;
      traverse(head, [&](Node<T> *node) {
         if (!found && node->val == value) { found = copySubtree(node); }
      });
      return fromNode(found);
   };
};
