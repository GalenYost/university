#include "tree_searcher.h"

#include <stdbool.h>
#include <stdio.h>

Node *preorderHelper(BinaryTree *tree, Node *node, int target) {
   if (!node) return NULL;

   if (node->value == target) {
      printf("Found %d, path: ", target);
      displayPath(tree);
      return node;
   }

   if (node->left) {
      movePtr(tree, LEFT);
      Node *next = preorderHelper(tree, node->left, target);
      if (next) return next;
      movePtr(tree, UP);
   }

   if (node->right) {
      movePtr(tree, RIGHT);
      Node *next = preorderHelper(tree, node->right, target);
      if (next) return next;
      movePtr(tree, UP);
   }

   return NULL;
}

Node *preorder(BinaryTree *tree, int target) {
   if (!tree || !tree->head) return NULL;

   movePtr(tree, HEAD);

   return preorderHelper(tree, tree->head, target);
}

Node *inorderHelper(BinaryTree *tree, Node *node, int target) {
   if (!node) return NULL;

   if (node->left) {
      movePtr(tree, LEFT);
      Node *next = inorderHelper(tree, node->left, target);
      if (next) return next;
      movePtr(tree, UP);
   }

   if (node->value == target) {
      printf("Found %d, path: ", target);
      displayPath(tree);
      return node;
   }

   if (node->right) {
      movePtr(tree, RIGHT);
      Node *next = inorderHelper(tree, node->right, target);
      if (next) return next;
      movePtr(tree, UP);
   }

   return NULL;
}

Node *inorder(BinaryTree *tree, int target) {
   if (!tree || !tree->head) return NULL;

   movePtr(tree, HEAD);

   return inorderHelper(tree, tree->head, target);
}

Node *postorderHelper(BinaryTree *tree, Node *node, int target) {
   if (!node) return NULL;

   if (node->left) {
      movePtr(tree, LEFT);
      Node *next = postorderHelper(tree, node->left, target);
      if (next) return next;
      movePtr(tree, UP);
   }

   if (node->right) {
      movePtr(tree, RIGHT);
      Node *next = postorderHelper(tree, node->right, target);
      if (next) return next;
      movePtr(tree, UP);
   }

   if (node->value == target) {
      printf("Found %d, path: ", target);
      displayPath(tree);
      return node;
   }

   return NULL;
}

Node *postorder(BinaryTree *tree, int target) {
   if (!tree || !tree->head) return NULL;

   movePtr(tree, HEAD);

   return postorderHelper(tree, tree->head, target);
}
