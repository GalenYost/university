pub mod hash;
pub mod input;

use hash::*;
use input::*;

fn main() {
    let division = || {
        let key = read_input_and_cast_value("Key: ", InputType::Number);
        let key = handle_value_cast_number(key);

        let size = read_input_and_cast_value("Size: ", InputType::Number);
        let size = handle_value_cast_number(size);

        let result = hash_by_division(key as u32, size as u32);
        println!("Hashed by division: {}", result);
    };

    let multiplication = || {
        let key = read_input_and_cast_value("Key: ", InputType::Number);
        let key = handle_value_cast_number(key);

        let size = read_input_and_cast_value("Size: ", InputType::Number);
        let size = handle_value_cast_number(size);

        let result = hash_by_multiplication(key as u32, size as u32);
        println!("Hashed by multiplication: {}", result);
    };

    let str_keys = || {
        let key = read_input_and_cast_value("Key: ", InputType::Text);
        let key = handle_value_cast_string(key);

        let size = read_input_and_cast_value("Size: ", InputType::Number);
        let size = handle_value_cast_number(size);

        let result = hash_by_string(key.as_str(), size as u32);
        println!("Hashed by string keys: {}", result);
    };

    let mut input_buffer = InputBuffer::new();
    input_buffer
        .bind("0", "exit", || std::process::exit(0))
        .bind("1", "division", division)
        .bind("2", "multiplication", multiplication)
        .bind("3", "string keys", str_keys);

    loop {
        println!("\nOptions:");
        input_buffer.prompt();
        input_buffer
            .read_input_and_call("Choice: ")
            .map_err(|e| println!("{}", e))
            .ok();
    }
}
