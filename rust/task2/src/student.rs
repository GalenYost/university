use crate::input::{
    InputType, handle_value_cast_number, handle_value_cast_string, read_input_and_cast_value,
};

pub enum SearchCriteria {
    Name(String),
    Surname(String),
    Address(String),
    Group(String),
    Rating(usize),
    Index(usize),
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Student {
    name: String,
    surname: String,
    address: String,
    code: String,
    rating: usize,
}

impl Student {
    pub fn new(name: &str, surname: &str, address: &str, code: &str) -> Self {
        Self {
            name: name.into(),
            surname: surname.into(),
            address: address.into(),
            code: code.into(),
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

    pub fn code(mut self, code: &str) -> Self {
        self.code = code.into();
        self
    }

    pub fn rating(mut self, value: usize) -> Self {
        self.rating = value.clamp(0, 100);
        self
    }

    pub fn get_info(&self) -> (&String, &String, &String, &String, &usize) {
        (
            &self.name,
            &self.surname,
            &self.address,
            &self.code,
            &self.rating,
        )
    }
}

pub fn prompt_student_info() -> (String, String, String, String, usize) {
    let name_input = read_input_and_cast_value("Student name: ", InputType::Text);
    let name = handle_value_cast_string(name_input);

    let surname_input = read_input_and_cast_value("Student surname: ", InputType::Text);
    let surname = handle_value_cast_string(surname_input);

    let address_input = read_input_and_cast_value("Student address: ", InputType::Text);
    let address = handle_value_cast_string(address_input);

    let code_input = read_input_and_cast_value("Student group code: ", InputType::Text);
    let code = handle_value_cast_string(code_input);

    let rating_input = read_input_and_cast_value("Student rating: ", InputType::Number);
    let rating = handle_value_cast_number(rating_input);

    (name, surname, address, code, rating)
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
        "code" => {
            let code = read_input_and_cast_value("Student group code: ", InputType::Text);
            let code = handle_value_cast_string(code);
            SearchCriteria::Group(code.into())
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
