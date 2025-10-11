#include "human.h"
#include "student.h"
#include <cstdio>

int main(void) {
   Human hm = Human("a", "b", 17);
   Student s1 = Student(hm, "abc-1", 99);

   printf("Dynamic polymorphism\n");
   Human *ptr = &hm;
   ptr->display();
   printf("\n");
   ptr = &s1;
   ptr->display();

   printf("\nStatic polymorphism\n");
   s1.setAvg(91);
   s1.display();
   printf("\n");
   s1.setAvg("76");
   s1.display();
   return 0;
}
