#pragma once

#include <iostream>

#include "vec.h"

enum class InputType {
    INT,
    BOOL,
    STR,
    CHAR,
};

struct InputValue {
    InputType type;
    int i;
    bool b;
    char ch;
    std::string str;
};

typedef struct {
    void *env;
    void (*cb)(void *);
} Closure;

typedef struct {
    char key;
    std::string value;
    Closure cl;
} Pair;

std::string readInput();
InputValue readInputCastValue(InputType type);
void to_lower_cpp(std::string &s);

class InputBuffer {
  private:
    Vector<Pair> pairs;

  public:
    InputBuffer();

    InputBuffer &bind(char key, std::string value, Closure cl);

    void prompt(std::string str);
    void awaitInput(std::string str);
};
