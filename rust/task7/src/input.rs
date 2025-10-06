use indexmap::IndexMap;
use std::io::{Write, stdin, stdout};

pub type Callback = Box<dyn FnMut() -> ()>;

#[derive(Eq, Hash, PartialEq)]
struct Bind {
    key: String,
    description: String,
}

pub struct InputBuffer {
    options: IndexMap<Bind, Callback>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum InputType {
    Text,
    Number,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum InputValue {
    Text(String),
    Number(usize),
}

impl std::fmt::Display for InputValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputValue::Text(text) => write!(f, "{}", text),
            InputValue::Number(num) => write!(f, "{}", num.to_string()),
        }
    }
}

impl Into<InputType> for InputValue {
    fn into(self) -> InputType {
        match self {
            InputValue::Text(_) => InputType::Text,
            InputValue::Number(_) => InputType::Number,
        }
    }
}

fn read_input() -> Result<String, String> {
    stdout()
        .flush()
        .map_err(|e| format!("flush error: {}", e))?;

    let mut inp_buf = String::new();
    stdin()
        .read_line(&mut inp_buf)
        .map_err(|e| format!("fail reading input: {}", e))?;

    Ok(inp_buf.trim().to_string())
}

pub fn handle_value_cast_string(inp: Result<InputValue, String>) -> String {
    match inp {
        Ok(InputValue::Text(txt)) => txt,
        Ok(_) => unreachable!(),
        Err(e) => {
            println!("{}", e);
            "".into()
        }
    }
}

pub fn handle_value_cast_number(inp: Result<InputValue, String>) -> usize {
    match inp {
        Ok(InputValue::Number(num)) => num,
        Ok(_) => unreachable!(),
        Err(e) => {
            println!("{}", e);
            0
        }
    }
}

pub fn read_input_and_cast_value(
    prompt_text: &str,
    input_type: InputType,
) -> Result<InputValue, String> {
    print!("{}", prompt_text);
    let input = read_input()?;
    Ok(match input_type {
        InputType::Text => InputValue::Text(input),
        InputType::Number => {
            let parsed: usize = input
                .parse()
                .map_err(|e| format!("invalid number input: {}", e))?;
            InputValue::Number(parsed)
        }
    })
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            options: IndexMap::new(),
        }
    }

    pub fn prompt(&self) -> () {
        for (bind, _) in self.options.iter() {
            println!("{} - {}", bind.key, bind.description);
        }
    }

    pub fn bind<F>(&mut self, key: &str, description: &str, callback: F) -> &mut Self
    where
        F: FnMut() + 'static,
    {
        self.options.insert(
            Bind {
                key: key.trim().to_string(),
                description: description.trim().to_string(),
            },
            Box::new(callback),
        );
        self
    }

    pub fn read_input_and_call(&mut self, prompt_text: &str) -> Result<(), String> {
        print!("{}", prompt_text);
        let key: String = read_input()?;

        for (bind, callback) in self.options.iter_mut() {
            if bind.key == key {
                callback();
                return Ok(());
            }
        }

        Err("invalid key input".into())
    }
}
