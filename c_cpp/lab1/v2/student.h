#pragma once

#include "human.h"
#include <string>

class Student : public Human {
private:
   std::string group_code;
   int avg;

public:
   Student(const std::string &name, const std::string &surname, int age, const std::string &code, int avg);

   std::string getCode() const;
   int getAvg() const;

   void setCode(const std::string &code);
   void setAvg(int avg);

   void display() const override;
};
