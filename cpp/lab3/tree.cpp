#include "tree.h"
#include "log.h"
#include <functional>

template <typename T> Node<T>::Node() = default;
template <typename T>
Node<T>::Node(const T &v) : val(v), left(nullptr), right(nullptr) {}

template struct Node<int>;
template struct Node<std::string>;

template <typename T> void collectValues(Node<T> *node, Vector<T> *vec) {
    if (!node) return;
    vec->push(node->val);
    collectValues(node->left, vec);
    collectValues(node->right, vec);
}

inline int cmpInt(const void *a, const void *b) {
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

template <typename T> Node<T> *copySubtree(Node<T> *node) {
    if (!node) return nullptr;
    Node<T> *newNode = new Node<T>(node->val);
    newNode->left = copySubtree(node->left);
    newNode->right = copySubtree(node->right);
    return newNode;
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

template <typename T> void BinaryTree<T>::clear(Node<T> *node) {
    if (!node) return;
    clear(node->left);
    clear(node->right);
    delete node;
}

template <typename T>
Node<T> *BinaryTree<T>::getNthNode(Node<T> *node, unsigned &index) const {
    if (!node) return nullptr;

    Node<T> *found = getNthNode(node->left, index);
    if (found) return found;

    if (index == 0) return node;
    index--;

    return getNthNode(node->right, index);
}

template <typename T>
Node<T> *BinaryTree<T>::getNodeAt(unsigned depth, unsigned index) const {
    if (!head) return nullptr;

    Vector<Node<T> *> current_level;
    current_level.push(head);

    for (unsigned d = 0; d < depth; ++d) {
        Vector<Node<T> *> next_level;

        for (unsigned i = 0; i < current_level.len(); ++i) {
            Node<T> **node_ptr = current_level.get(i);
            if (!node_ptr || !(*node_ptr)) continue;

            Node<T> *node = *node_ptr;

            if (node->left) { next_level.push(node->left); }
            if (node->right) { next_level.push(node->right); }
        }

        if (next_level.empty()) return nullptr;
        current_level = next_level;
    }

    if (index >= current_level.len()) return nullptr;

    Node<T> **result = current_level.get(index);
    return result && *result ? *result : nullptr;
}

template <typename T> Node<T> *BinaryTree<T>::safeGetLastPath() const {
    if (path.len() == 0) return nullptr;
    Node<T> **last = path.get(path.len() - 1);
    return last ? *last : nullptr;
}

template <typename T> void BinaryTree<T>::deleteSubtree(Node<T> *node) {
    if (!node) return;
    deleteSubtree(node->left);
    deleteSubtree(node->right);
    delete node;
}

template <typename T>
Node<T> *BinaryTree<T>::findParent(Node<T> *root, Node<T> *child) const {
    if (!root || !child) return nullptr;
    if (root->left == child || root->right == child) return root;
    Node<T> *left = findParent(root->left, child);
    if (left) return left;
    return findParent(root->right, child);
}

template <typename T>
Node<T> *BinaryTree<T>::cloneSubtree(const Node<T> *src) const {
    if (!src) return nullptr;
    Node<T> *newNode = new Node<T>(src->val);
    newNode->left = cloneSubtree(src->left);
    newNode->right = cloneSubtree(src->right);
    return newNode;
}

template <typename T>
void BinaryTree<T>::replaceSubtree(Node<T> *&target, Node<T> *source) {
    deleteSubtree(target);
    target = cloneSubtree(source);
}

template <typename T>
void BinaryTree<T>::displayIndented(std::ostream &out, Node<T> *node,
                                    int depth) const {
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

template <typename T>
void BinaryTree<T>::saveNode(std::ostream &out, Node<T> *node) const {
    if (!node) {
        out << "# ";
        return;
    }
    out << node->val << " ";
    saveNode(out, node->left);
    saveNode(out, node->right);
}

template <typename T> Node<T> *BinaryTree<T>::loadNode(std::istream &in) const {
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

template <typename T> BinaryTree<T>::BinaryTree() = default;
template <typename T> BinaryTree<T>::BinaryTree(const BinaryTree &other) {
    head = copySubtree(other.head);
}
template <typename T>
BinaryTree<T>::BinaryTree(Node<T> *node) : head(node), cur_ptr(node) {}

template <typename T> BinaryTree<T>::~BinaryTree() {
    clear(head);
    head = nullptr;
    cur_ptr = nullptr;
}

template <typename T> const T &BinaryTree<T>::operator[](unsigned n) const {
    Node<T> *result = getNthNode(head, n);
    if (!result) {
        log(LogLevel::ERROR, "BinaryTree index out of range");
        std::exit(0);
    }
    return result->val;
}

template <typename T>
BinaryTree<T> &BinaryTree<T>::operator=(const BinaryTree &other) {
    if (this == &other) return *this;
    clear(head);
    head = copySubtree(other.head);
    return *this;
}

template <typename T>
BinaryTree<T>::BinaryTree(BinaryTree<T> &&other) noexcept {
    head = other.head;
    cur_ptr = other.cur_ptr;
    other.head = nullptr;
    other.cur_ptr = nullptr;
}

template <typename T>
BinaryTree<T> &BinaryTree<T>::operator=(BinaryTree<T> &&other) noexcept {
    if (this == &other) return *this;
    clear(head);
    head = other.head;
    cur_ptr = other.cur_ptr;
    other.head = nullptr;
    other.cur_ptr = nullptr;
    return *this;
}

template <typename T>
std::ostream &operator<<(std::ostream &out, BinaryTree<T> const &tree) {
    if (&out == &std::cout || &out == &std::cerr) {
        tree.displayIndented(out, tree.head, 0);
    } else {
        tree.saveNode(out, tree.head);
    }
    return out;
}

template <typename T>
std::istream &operator>>(std::istream &in, BinaryTree<T> &tree) {
    tree.head = tree.loadNode(in);
    tree.cur_ptr = tree.head;
    return in;
}

template <typename T>
BinaryTree<T> &BinaryTree<T>::operator+(std::pair<T, Direction> p) {
    Node<T> *newNode = new Node<T>(p.first);

    if (!cur_ptr) *this ^ Direction::HEAD;
    if (!cur_ptr) {
        log(LogLevel::WARN, "No head, forcing insert to head value: " +
                                std::to_string(p.first));
        p.second = Direction::HEAD;
    }

    switch (p.second) {
    case Direction::LEFT:
        replaceSubtree(cur_ptr->left, newNode);
        break;
    case Direction::RIGHT:
        replaceSubtree(cur_ptr->right, newNode);
        break;
    case Direction::HEAD: {
        if (!head) {
            Node<T> *new_head = new Node<T>(p.first);
            head = new_head;
        } else {
            head->val = p.first;
        }
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
template <typename T>
BinaryTree<T> &BinaryTree<T>::operator+(std::pair<T, std::string> p) {
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

template <typename T> BinaryTree<T> &BinaryTree<T>::operator^(Direction dir) {
    if (!cur_ptr) {
        log(LogLevel::WARN, "No current pointer, forcing pointer to head");
        if (!head) log(LogLevel::WARN, "No head found, infinite loop possible");
        cur_ptr = head;
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
template <typename T>
BinaryTree<T> &BinaryTree<T>::operator^(const std::string &dir) {
    if (!cur_ptr) {
        log(LogLevel::WARN,
            "Dropping call, no current pointer, set pointer to head");
        cur_ptr = head;
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

template <typename T>
BinaryTree<T>::NodeRef::NodeRef(BinaryTree<T> *t, Node<T> *n)
    : tree(t), node(n) {}

template <typename T>
typename BinaryTree<T>::NodeRef &
BinaryTree<T>::NodeRef::operator=(const T &new_val) {
    if (!node) {
        log(LogLevel::WARN, "Attempted to assign to non-existent node");
        return *this;
    }
    node->val = new_val;
    return *this;
}

template <typename T>
typename BinaryTree<T>::NodeRef &
BinaryTree<T>::NodeRef::operator=(const BinaryTree<T> &other) {
    if (!node) {
        log(LogLevel::WARN, "Attempted to replace non-existent node");
        return *this;
    }
    tree->replaceSubtree(node, other.head);
    return *this;
}

template <typename T> BinaryTree<T>::NodeRef::operator T &() {
    return node->val;
}

template <typename T> BinaryTree<T>::NodeRef::operator const T &() const {
    return node->val;
}

template <typename T>
typename BinaryTree<T>::NodeRef BinaryTree<T>::operator()(unsigned depth,
                                                          unsigned index) {
    Node<T> *node = getNodeAt(depth, index);
    return NodeRef(this, node);
}

template <typename T> void BinaryTree<T>::sortTree(bool ascending) {
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

template <typename T> bool BinaryTree<T>::empty() const {
    return head == nullptr;
}

template <typename T> void BinaryTree<T>::debug_print_inorder() const {
#ifdef DEBUG_MODE
    unsigned i = 0;
    std::function<void(Node<T> *)> pr = [&](Node<T> *node) {
        if (!node) return;
        pr(node->left);
        std::cerr << "[" << i++ << "] = " << node->val << "\n";
        pr(node->right);
    };
    pr(head);
#endif
}

template class BinaryTree<int>;
template class BinaryTree<char>;

template std::ostream &operator<<(std::ostream &, BinaryTree<int> const &);
template std::ostream &operator<<(std::ostream &, BinaryTree<char> const &);

template std::istream &operator>>(std::istream &, BinaryTree<int> &);
template std::istream &operator>>(std::istream &, BinaryTree<char> &);
