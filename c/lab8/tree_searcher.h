#ifndef TREE_SEARCHER_H
#define TREE_SEARCHER_H

#include "tree.h"

#include <stdbool.h>

Node *preorder(BinaryTree *tree, int target);
Node *inorder(BinaryTree *tree, int target);
Node *postorder(BinaryTree *tree, int target);

#endif
