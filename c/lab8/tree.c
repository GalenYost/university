#include "tree.h"
#include "log.h"

#include <stdio.h>
#include <stdlib.h>

Node *initNode() {
   Node *node = malloc(sizeof(Node));
   if (!node) {
#ifdef DEBUG_MODE
      log(ERROR, "Fail allocating memory for Node");
#endif
      return NULL;
   }
   node->left = NULL;
   node->right = NULL;
   return node;
}

BinaryTree *initBinaryTree() {
   BinaryTree *bt = malloc(sizeof(BinaryTree));
   if (!bt) {
#ifdef DEBUG_MODE
      log(ERROR, "Fail allocating memory for BinaryTree");
#endif
      return NULL;
   }
   Node *node = initNode();
   if (!node) {
      free(bt);
      return NULL;
   }

   bt->head = node;
   bt->ptr = bt->head;
   bt->path_len = 0;
   return bt;
}

BinaryTree *initBinaryTreeWithValue(int value) {
   BinaryTree *bt = initBinaryTree();
   bt->head->value = value;
   bt->path_len++;
   bt->path = malloc(bt->path_len * sizeof(Node *));
   bt->path[0] = bt->head;
   return bt;
}

BinaryTree *constructBinaryTreeFromNode(Node *node) {
   BinaryTree *bt;
   bt->head = node;
   bt->ptr = bt->head;
   return bt;
}

void displayIndented(BinaryTree *bt, Node *node, int depth) {
   if (!node) return;

   displayIndented(bt, node->right, depth + 1);
   for (int i = 0; i < depth; ++i) printf("\t");

   if (node == bt->ptr) {
      printf("%s %d %s\n", DEBUG_COLOR, node->value, RESET_COLOR);
   } else {
      printf("%d\n", node->value);
   }
   displayIndented(bt, node->left, depth + 1);
}

void clearRecursive(Node *node) {
   if (!node) return;
   clearRecursive(node->left);
   clearRecursive(node->right);
   node->left = node->right = NULL;
   free(node);
}

void clear(BinaryTree *bt) {
   clearRecursive(bt->head);
   free(bt);
}

void displayPath(BinaryTree *tree) {
   if (!tree || !tree->path || tree->path_len == 0) {
      printf("(empty path)\n");
      return;
   }

   for (int i = 0; i < tree->path_len; i++) {
      Node *el = tree->path[i];
      if (!el) break;

      printf("%d", el->value);
      if (i < tree->path_len - 1 && tree->path[i + 1]) { printf("->"); }
   }
   printf("\n");
}

void display(BinaryTree *tree) {
   if (!tree->head) {
      log(INFO, "Tree is empty");
      return;
   }
   log(INFO, "Current tree structure:");
   displayIndented(tree, tree->head, 0);
}

void insertHead(BinaryTree *tree, int val) {
   if (tree->head) { clear(tree); }
   tree = initBinaryTreeWithValue(val);
}

void insertUnderPtr(BinaryTree *tree, Direction dir, int val) {
   if (!tree->head) {
#ifdef DEBUG_MODE
      log(WARN, "Tree is empty, inserting new head");
#endif
      insertHead(tree, val);
      return;
   }
   if (!tree->ptr) { tree->ptr = tree->head; }

   switch (dir) {
   case LEFT:
      if (!tree->ptr) { return; }
      Node *node_left = initNode();
      node_left->value = val;
      tree->ptr->left = node_left;
      break;
   case RIGHT:
      if (!tree->ptr) { return; }
      Node *node_right = initNode();
      node_right->value = val;
      tree->ptr->right = node_right;
      break;
   default:
      log(ERROR, "Only LEFT and RIGHT are possible");
      break;
   };
}

void removeUnderPtr(BinaryTree *tree, Direction dir) {
   if (!tree->head) {
      log(ERROR, "Tree is empty");
      return;
   }
   if (!tree->ptr) {
      log(ERROR, "No pointer");
      return;
   }

   switch (dir) {
   case LEFT:
      clearRecursive(tree->ptr->left);
      break;
   case RIGHT:
      clearRecursive(tree->ptr->right);
      break;
   default:
      log(ERROR, "Only LEFT and RIGHT are possible");
      break;
   };
}

void movePtr(BinaryTree *tree, Direction dir) {
   if (!tree || !tree->head) {
      log(ERROR, "Tree is empty");
      return;
   }

   if (!tree->ptr) {
#ifdef DEBUG_MODE
      log(WARN, "Pointer is NULL, resetting to head");
#endif
      dir = HEAD;
   }

   switch (dir) {
   case HEAD:
      tree->ptr = tree->head;
      free(tree->path);
      tree->path = malloc(sizeof(Node *));
      if (!tree->path) {
         log(ERROR, "Failed to allocate path");
         return;
      }
      tree->path[0] = tree->head;
      tree->path_len = 1;
      break;

   case UP:
      if (tree->path_len < 2) {
#ifdef DEBUG_MODE
         log(WARN, "Already at root, cant move UP");
#endif
         return;
      }
      tree->path_len--;
      tree->ptr = tree->path[tree->path_len - 1];
      tree->path = realloc(tree->path, tree->path_len * sizeof(Node *));
      break;

   case LEFT:
      if (!tree->ptr || !tree->ptr->left) {
         log(ERROR, "Left child does not exist");
         return;
      }
      tree->ptr = tree->ptr->left;
      tree->path_len++;
      tree->path = realloc(tree->path, tree->path_len * sizeof(Node *));
      if (!tree->path) {
         log(ERROR, "Failed to expand path");
         return;
      }
      tree->path[tree->path_len - 1] = tree->ptr;
      break;

   case RIGHT:
      if (!tree->ptr || !tree->ptr->right) {
         log(ERROR, "Right child does not exist");
         return;
      }
      tree->ptr = tree->ptr->right;
      tree->path_len++;
      tree->path = realloc(tree->path, tree->path_len * sizeof(Node *));
      if (!tree->path) {
         log(ERROR, "Failed to expand path");
         return;
      }
      tree->path[tree->path_len - 1] = tree->ptr;
      break;
   }
}

Node *find(BinaryTree *tree, SearchOrder order, int val) {}
