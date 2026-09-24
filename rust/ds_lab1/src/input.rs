use indexmap::IndexMap;
use std::fmt;
use std::io::{self, stdin, stdout, Write};

pub type Callback = Box<dyn FnMut()>;

pub struct MenuOption {
    pub description: String,
    pub callback: Callback,
}

#[derive(Default)]
pub struct InputBuffer {
    options: IndexMap<String, MenuOption>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputValue {
    Text(String),
    Number(isize),
}

impl InputValue {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<isize> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }
}

impl fmt::Display for InputValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{text}"),
            Self::Number(num) => write!(f, "{num}"),
        }
    }
}

fn read_input() -> io::Result<String> {
    stdout().flush()?;
    let mut inp_buf = String::new();
    stdin().read_line(&mut inp_buf)?;
    Ok(inp_buf.trim().to_string())
}

pub fn read_as<T: std::str::FromStr>(prompt: &str) -> Result<T, String>
where
    T::Err: std::fmt::Display,
{
    print!("{prompt}");
    let input = read_input().map_err(|e| format!("failed reading input: {e}"))?;
    input.parse::<T>().map_err(|e| format!("parse error: {e}"))
}

impl InputBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prompt(&self) {
        for (key, option) in &self.options {
            println!("{key} - {}", option.description);
        }
    }

    pub fn bind<F>(&mut self, key: &str, description: &str, callback: F) -> &mut Self
    where
        F: FnMut() + 'static,
    {
        self.options.insert(
            key.trim().to_string(),
            MenuOption {
                description: description.trim().to_string(),
                callback: Box::new(callback),
            },
        );
        self
    }

    pub fn read_input_and_call(&mut self, prompt_text: &str) -> Result<(), String> {
        print!("{prompt_text}");
        let key = read_input().map_err(|e| format!("failed reading input: {e}"))?;

        if let Some(option) = self.options.get_mut(&key) {
            (option.callback)();
            Ok(())
        } else {
            Err(format!("invalid key input: '{key}'"))
        }
    }
}
