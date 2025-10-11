#pragma once

#include "human.h"
#include <string>

class Student : public Human {
 private:
   std::string group_code;
   int avg;

 public:
   Student(Human human, const std::string &code, int avg);

   std::string getCode() const;
   int getAvg() const;

   void setCode(const std::string &code);
   void setAvg(int avg);
   void setAvg(std::string avg);

   void display() const;
};
