#pragma once

#include "input.h"
#include "vec.h"

enum class Direction {
   UP,
   LEFT,
   RIGHT,
   HEAD,
};

template <typename T> struct Node {
   T val;
   Node *left;
   Node *right;
};

template <typename T> void collectValues(Node<T> *node, Vector<T> *vec) {
   if (!node) return;

   int *v = (int *)malloc(sizeof(int));
   *v = node->val;
   vec->push(v);

   collectValues(node->left, vec);
   collectValues(node->right, vec);
}

int cmpInt(const void *a, const void *b) {
   int x = **(int **)a;
   int y = **(int **)b;
   return x - y;
}

template <typename T> Node<T> *buildBST(Vector<T> *vec, int l, int r) {
   if (l > r) return NULL;

   int mid = l + (r - l) / 2;
   Node<T> *node = (Node<T> *)malloc(sizeof(Node<T>));
   node->val = *(int *)vec->get(mid);
   node->left = buildBST(vec, l, mid - 1);
   node->right = buildBST(vec, mid + 1, r);

   return node;
}

template <typename T> struct FindCtx {
   int value;
   Node<T> *found;
};

template <typename T> Node<T> *copySubtree(Node<T> *node) {
   if (!node) return nullptr;
   Node<T> *newNode = new Node<T>{node->val};
   newNode->left = copySubtree(node->left);
   newNode->right = copySubtree(node->right);
   return newNode;
}

template <typename T> void findCallback(void *env, Node<T> *node) {
   FindCtx<T> *ctx = (struct FindCtx<T> *)env;
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
   };

   Node<T> *getNthNode(Node<T> *node, int &index) const {
      if (!node) return nullptr;

      Node<T> *left = getNthNode(node->left, index);
      if (left) return left;

      if (index == 0) return node;
      index--;

      return getNthNode(node->right, index);
   }

   void traverse(Node<T> *node, Closure cl) {
      if (!node) return;

      traverse(node->left, cl);
      cl.cb(cl.env);
      traverse(node->right, cl);
   };

   void displayIndented(Node<T> *node, int depth) {
      if (!node) return;

      displayIndented(node->right, depth + 1);
      for (int i = 0; i < depth; ++i) printf("\t");

      if (node == cur_ptr) {
         printf("%s%d%s\n", DEBUG_COLOR, node->val, RESET_COLOR);
      } else {
         printf("%d\n", node->val);
      }
      displayIndented(node->left, depth + 1);
   }

   void saveNode(FILE *out, Node<T> *node) {
      if (!node) {
         fprintf(out, "# ");
         return;
      }
      fprintf(out, "%d ", node->val);
      saveNode(out, node->left);
      saveNode(out, node->right);
   }

   Node<T> *loadNode(FILE *in) {
      char token[64];
      if (fscanf(in, "%63s", token) != 1) {
         log(LogLevel::ERROR, "EOF");
         return NULL;
      }

      if (token[0] == '#' && token[1] == '\0') return NULL;

      int value;
      if (sscanf(token, "%d", &value) != 1) {
         log(LogLevel::ERROR, "Parse error");
         return NULL;
      }

      Node<T> *node = (Node<T> *)malloc(sizeof(Node<T>));
      if (!node) {
         log(LogLevel::ERROR, "fail allocating memory");
         return NULL;
      }

      node->val = value;
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
      delete &path;
   }

   const T &operator[](int n) const {
      int idx = n;
      Node<T> *result = getNthNode(head, idx);
      if (!result) {
         log(LogLevel::ERROR, "BinaryTree index out of range");
         return NULL;
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
            return true;
         } else {
            movePtr(Direction::HEAD);
         }
      }

      switch (dir) {
      case Direction::LEFT:
         if (cur_ptr->left) clear(cur_ptr->left);
         cur_ptr->left = new Node<T>{value, NULL, NULL};
         break;
      case Direction::RIGHT:
         if (cur_ptr->right) clear(cur_ptr->right);
         cur_ptr->right = new Node<T>{value, NULL, NULL};
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
      case Direction::UP:
         if (path.len() == 0) {
            cur_ptr = *path.get(path.len() - 1);
            path.pop();
         } else {
            log(LogLevel::WARN, "Dropping call, path is empty");
         }
         break;

      case Direction::LEFT:
         if (cur_ptr->left) {
            path.push(&cur_ptr);
            cur_ptr = cur_ptr->left;
         } else {
            log(LogLevel::WARN, "Left element doesnt exist");
         }
         break;

      case Direction::RIGHT:
         if (cur_ptr->right) {
            path.push(&cur_ptr);
            cur_ptr = cur_ptr->right;
         } else {
            log(LogLevel::WARN, "Right element doesnt exist");
         }
         break;

      case Direction::HEAD:
         cur_ptr = head;
         delete &path;
         break;
      };

      return;
   }

   void iter(Node<T> *node, Closure closure) {
      if (!node) return;
      closure.cb(node, closure.env);
      iter(node->left, closure);
      iter(node->right, closure);
   }

   void displayCurrent() {
      if (this->empty()) {
         log(LogLevel::INFO, "Tree is empty");
         return;
      }
      if (!cur_ptr) { movePtr(Direction::HEAD); }
      log(LogLevel::INFO, "Current pointer value: %s", cur_ptr->val);
   };
   void displayTree() {
      if (this->empty()) {
         log(LogLevel::INFO, "Tree is empty");
         return;
      }
      log(LogLevel::INFO, "Current tree structure:");
      displayIndented(head, 0);
   };

   void saveToFile(const char *filename, Node<T> *head) {
      FILE *f = fopen(filename, "w");
      if (!f) {
         log(LogLevel::ERROR, "Cannot open file for writing: %s\n", filename);
         return;
      }
      saveNode(f, head);
      fclose(f);
   }

   Node<T> *loadFromFile(const char *filename, Node<T> **cur_ptr) {
      FILE *f = fopen(filename, "r");
      if (!f) {
         log(LogLevel::ERROR, "Cannot open file for reading: %s\n", filename);
         return NULL;
      }

      Node<T> *head = loadNode(f);
      fclose(f);

      if (cur_ptr) *cur_ptr = head;
      return head;
   }

   void sortTree(Node<T> **head) {
      Vector<T> values = Vector<T>();
      collectValues(*head, &values);

      qsort(values.vec, values.len(&values), sizeof(int *), cmpInt);

      clear(*head);
      *head = buildBST(&values, 0, values.len(&values) - 1);

      delete values;
   }

   BinaryTree<T> find(T value) {
      Node<T> *found;
      FindCtx ctx = {.value = value, .found = NULL};
      traverse(head, findCallback, &ctx);
      return fromNode(ctx.found);
   };

   bool empty() const { return head == NULL; }
};
