#include <string>
#include <cstdio>

#include "human.h"

Human::Human(const std::string &name, const std::string &surname, int age) : name(name), surname(surname), age(age) {};

std::string Human::getName() const { return name; }
std::string Human::getSurname() const { return surname; }
int Human::getAge() const { return age; }

void Human::setName(const std::string &n) {
   name = n;
};
void Human::setSurname(const std::string &sn) {
   surname = sn;
};
void Human::setAge(int a) {
   if (a > 0 && a < 150) age = a;
};

void Human::display() const {
   printf("Full name: %s %s\n", getName().c_str(), getSurname().c_str());
   printf("Age: %d\n", getAge());
};
