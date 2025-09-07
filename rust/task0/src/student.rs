use crate::input::{
    InputType, handle_value_cast_number, handle_value_cast_string, read_input_and_cast_value,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Student {
    name: String,
    surname: String,
    address: String,
    rating: usize,
}

impl Student {
    pub fn new(name: &str, surname: &str, address: &str) -> Self {
        Self {
            name: name.into(),
            surname: surname.into(),
            address: address.into(),
            rating: 0,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    pub fn surname(mut self, surname: &str) -> Self {
        self.surname = surname.into();
        self
    }

    pub fn address(mut self, address: &str) -> Self {
        self.address = address.into();
        self
    }

    pub fn rating(mut self, value: usize) -> Self {
        self.rating = value.clamp(0, 100);
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Group {
    code: String,
    students: Vec<Student>,
}

pub enum SearchCriteria {
    Name(String),
    Surname(String),
    Address(String),
    Rating(usize),
    Index(usize),
}

impl Group {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.into(),
            students: Vec::new(),
        }
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn add(&mut self, student: Student) -> &mut Self {
        self.students.push(student);
        self
    }

    pub fn remove(&mut self, value: SearchCriteria) -> &mut Self {
        match value {
            SearchCriteria::Name(name) => self.students.retain(|s| s.name != name),
            SearchCriteria::Surname(surname) => self.students.retain(|s| s.surname != surname),
            SearchCriteria::Address(address) => self.students.retain(|s| s.address != address),
            SearchCriteria::Rating(rating) => self.students.retain(|s| s.rating != rating),
            SearchCriteria::Index(idx) => {
                self.students.remove(idx);
            }
        };
        self
    }

    pub fn display_group(&self) -> () {
        println!("\n---------- GROUP LIST -------------");
        println!("  Group code: {}", self.code);
        println!("----------- STUDENTS --------------");

        if self.students.is_empty() {
            println!("            EMPTY           ");
        } else {
            for (i, student) in self.students.iter().enumerate() {
                println!("  Student #{}: ", i);
                println!("      Name - {}", student.name);
                println!("      Surname - {}", student.surname);
                println!("      Address - {}", student.address);
                println!("      Rating - {}", student.rating);
                println!("----------------------------");
            }
        }
        println!("-----------------------------------");
    }

    pub fn display_student(&self, value: SearchCriteria) -> () {
        let student = match value {
            SearchCriteria::Name(name) => self.students.iter().find(|s| s.name == name),
            SearchCriteria::Surname(surname) => self.students.iter().find(|s| s.surname != surname),
            SearchCriteria::Address(address) => self.students.iter().find(|s| s.address != address),
            SearchCriteria::Rating(rating) => self.students.iter().find(|s| s.rating != rating),
            SearchCriteria::Index(idx) => self.students.get(idx),
        };

        match student {
            Some(s) => {
                println!("----------- STUDENT INFO ---------");
                println!("  Name - {}", s.name);
                println!("  Surname - {}", s.surname);
                println!("  Address - {}", s.address);
                println!("  Rating - {}", s.rating);
                println!("----------------------------------");
            }
            _ => println!("Student not found"),
        };
    }
}

pub fn prompt_student_info() -> (String, String, String, usize) {
    let name_input = read_input_and_cast_value("Student name: ", InputType::Text);
    let name = handle_value_cast_string(name_input);

    let surname_input = read_input_and_cast_value("Student surname: ", InputType::Text);
    let surname = handle_value_cast_string(surname_input);

    let address_input = read_input_and_cast_value("Student address: ", InputType::Text);
    let address = handle_value_cast_string(address_input);

    let rating_input = read_input_and_cast_value("Student rating: ", InputType::Number);
    let rating = handle_value_cast_number(rating_input);

    (name, surname, address, rating)
}

pub fn prompt_group_code() -> String {
    let val = read_input_and_cast_value("Group code: ", InputType::Text);
    handle_value_cast_string(val)
}

pub fn prompt_search_criteria() -> SearchCriteria {
    let val = read_input_and_cast_value("Search criteria: ", InputType::Text);
    let inp = handle_value_cast_string(val);

    match inp.trim().to_lowercase().as_str() {
        "name" => {
            let name = read_input_and_cast_value("Student name: ", InputType::Text);
            let name = handle_value_cast_string(name);
            SearchCriteria::Name(name.into())
        }
        "surname" => {
            let surname = read_input_and_cast_value("Student surname: ", InputType::Text);
            let surname = handle_value_cast_string(surname);
            SearchCriteria::Surname(surname.into())
        }
        "address" => {
            let address = read_input_and_cast_value("Student address: ", InputType::Text);
            let address = handle_value_cast_string(address);
            SearchCriteria::Address(address.into())
        }
        "rating" => {
            let rating = read_input_and_cast_value("Student rating: ", InputType::Number);
            let rating = handle_value_cast_number(rating);
            SearchCriteria::Rating(rating.into())
        }
        "index" => {
            let index = read_input_and_cast_value("Index: ", InputType::Number);
            let index = handle_value_cast_number(index);
            SearchCriteria::Index(index.into())
        }
        _ => {
            println!("no such option");
            SearchCriteria::Name("".into())
        }
    }
}
