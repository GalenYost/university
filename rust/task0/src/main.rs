#[derive(Debug, Clone, Eq, PartialEq)]
struct Student<'a> {
    name: &'a str,
    surname: &'a str,
    address: &'a str,
    rating: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Group<'b> {
    code: &'b str,
    students: Vec<Student<'b>>,
}

enum SearchCriteria<'a> {
    Name(&'a str),
    Surname(&'a str),
    Rating(u64),
}

impl<'b> Group <'b> {
    fn add(&mut self, student: Student<'b>) -> () {
        self.students.push(student);
    }

    fn remove(&mut self, value: SearchCriteria<'b>) -> () {
        match value {
            SearchCriteria::Name(name) => self.students.retain(|s| s.name != name),
            SearchCriteria::Surname(surname) => self.students.retain(|s| s.surname != surname),
            SearchCriteria::Rating(rating) => self.students.retain(|s| s.rating != rating),
        }
    }

    fn display_group(&self) -> () {
        println!("Group code: {}", self.code);

        for (i, student) in self.students.iter().enumerate() {
            println!(" Student #{}: ", i);
            println!("   Name - {}", student.name);
            println!("   Surname - {}", student.surname);
            println!("   Address - {}", student.address);
            println!("   Rating - {}", student.rating);
            println!("----------------------------");
        }
    }

    fn display_student(&self, value: SearchCriteria<'b>) -> () {
        let student = match value {
            SearchCriteria::Name(name) => self.students.iter().find(|s| s.name == name),
            SearchCriteria::Surname(surname) => self.students.iter().find(|s| s.surname != surname),
            SearchCriteria::Rating(rating) => self.students.iter().find(|s| s.rating != rating),
        };

        match student {
            Some(s) => {
                println!("Student info: ");
                println!("Name - {}", s.name);
                println!("Surname - {}", s.surname);
                println!("Address - {}", s.address);
                println!("Rating - {}", s.rating);
            },
            _ => println!("Not found"),
        };
    }
}

fn main() {
    let mut group1 = Group {
        code: "1-2-3",
        students: Vec::new(),
    };

    group1.add(Student {
        name: "a",
        surname: "a",
        address: "1234",
        rating: 95,
    });
    group1.add(Student {
        name: "b",
        surname: "b",
        address: "12345",
        rating: 94,
    });
    group1.add(Student {
        name: "c",
        surname: "c",
        address: "123456",
        rating: 72,
    });

    println!("--------------- GROUP ---------------");
    group1.display_group();
    println!("--------------- STUDENT ---------------");
    group1.display_student(SearchCriteria::Name("b"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adding_students() {
        let mut group1 = Group {
            code: "1-2-3",
            students: Vec::new(),
        };

        group1.add(Student {
            name: "a",
            surname: "a",
            address: "123456",
            rating: 95
        });

        assert_eq!(
            group1.students[0],
            Student {
                name: "a",
                surname: "a",
                address: "123456",
                rating: 95,
            }
        );

        group1.add(Student {
            name: "b",
            surname: "b",
            address: "123456",
            rating: 94
        });

        assert_ne!(
            group1.students[1],
            Student {
                name: "a",
                surname: "a",
                address: "123456",
                rating: 95,
            }
        );
    }

    #[test]
    #[should_panic]
    fn remove_panic() {
        let mut group1 = Group {
            code: "1-2-3",
            students: Vec::new(),
        };
        let removed = group1.students.remove(0);

        println!("{:?}", removed);
    }

    #[test]
    fn remove() {
        let mut group1 = Group {
            code: "1-2-3",
            students: Vec::new(),
        };

        group1.add(Student {
            name: "a",
            surname: "a",
            address: "123456",
            rating: 95
        });
        group1.add(Student {
            name: "b",
            surname: "b",
            address: "123456",
            rating: 94
        });

        group1.remove(SearchCriteria::Name("a"));
        assert!(group1.students.len() == 1);
    }
}
