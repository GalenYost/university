#pragma once

#include <string>

class Human {
private:
   std::string name;
   std::string surname;
   int age;

public:
   Human(const std::string &name, const std::string &surname, int age);

   std::string getName() const;
   std::string getSurname() const;
   int getAge() const;

   void setName(const std::string &name);
   void setSurname(const std::string &surname);
   void setAge(int age);

   virtual void display() const;
};
