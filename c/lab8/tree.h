#ifndef TREE_H
#define TREE_H

typedef enum {
   HEAD,
   UP,
   LEFT,
   RIGHT,
} Direction;

typedef enum {
   PREORDER,
   INORDER,
   POSTORDER,
} SearchOrder;

typedef struct Node {
   int value;
   struct Node *left;
   struct Node *right;
} Node;

typedef struct {
   Node *head;
   Node *ptr;
   Node **path;
   int path_len;
   int path_cap;
} BinaryTree;

Node *initNode();

BinaryTree *initBinaryTree();
BinaryTree *initBinaryTreeWithValue(int value);
BinaryTree *constructBinaryTreeFromNode(Node *node);

void display(BinaryTree *tree);
void displayPath(BinaryTree *tree);
void clear(BinaryTree *bt);

void insertHead(BinaryTree *tree, int val);
void insertUnderPtr(BinaryTree *tree, Direction dir, int val);
void removeUnderPtr(BinaryTree *tree, Direction dir);

void movePtr(BinaryTree *tree, Direction dir);

Node *find(BinaryTree *tree, SearchOrder order, int val);

#endif
