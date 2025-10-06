#include <cstdio>
#include <string>

#include "student.h"

Student::Student(
   const std::string &n,
   const std::string &sn,
   int age,
   const std::string &code,
   int avg
): Human(n, sn, age), group_code(code), avg(avg) {};

std::string Student::getCode() const { return group_code; }
int Student::getAvg() const { return avg; }

void Student::setCode(const std::string &code) { group_code = code; }
void Student::setAvg(int a) { if (a >= 0 && a <= 100) avg = a; }

void Student::display() const {
   printf("Full name: %s %s\n", getName().c_str(), getSurname().c_str());
   printf("Age: %d\n", getAge());
   printf("Group code: %s\n", getCode().c_str());
   printf("Average: %d\n", getAvg());
}
