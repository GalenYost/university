use indexmap::IndexMap;
use std::{
    io::{Write, stdin, stdout},
    str::FromStr,
};

pub type Callback = Box<dyn FnMut() -> ()>;

#[derive(Eq, Hash, PartialEq)]
struct Bind {
    key: String,
    description: String,
}

pub struct InputBuffer {
    options: IndexMap<Bind, Callback>,
}

fn read_input() -> String {
    stdout().flush().unwrap();

    let mut inp_buf = String::new();
    stdin()
        .read_line(&mut inp_buf)
        .map_err(|e| format!("fail reading input: {}", e))
        .ok();

    inp_buf.trim().to_string()
}

pub fn read_input_and_cast_value<T>(prompt_text: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    print!("{}", prompt_text);
    let input = read_input().to_string();
    input.trim().parse::<T>().unwrap()
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
