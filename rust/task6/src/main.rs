pub mod hash;
pub mod input;

use hash::*;
use input::*;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let table = Rc::new(RefCell::new(HashTable::new(20)));

    let table_insert = table.clone();
    let insert = move || {
        let key = read_input_and_cast_value("Key: ", InputType::Text);
        let key = handle_value_cast_string(key);

        let value = read_input_and_cast_value("Value: ", InputType::Text);
        let value = handle_value_cast_string(value);

        table_insert
            .borrow_mut()
            .insert(InputValue::Text(key), value);
    };

    let table_remove = table.clone();
    let remove = move || {
        let key = read_input_and_cast_value("Key: ", InputType::Number);
        let key = handle_value_cast_string(key);

        table_remove.borrow_mut().remove(InputValue::Text(key));
    };

    let table_find = table.clone();
    let find = move || {
        let key = read_input_and_cast_value("Key: ", InputType::Text);
        let key = handle_value_cast_string(key);

        if let Some(result) = table_find.borrow().find(InputValue::Text(key)) {
            println!("Result: {}", result);
        } else {
            println!("Not found");
        }
    };

    let debug_table = table.clone();
    let print = move || {
        let tbl = debug_table.borrow();
        println!("{}", tbl);
    };

    let mut input_buffer = InputBuffer::new();
    input_buffer
        .bind("0", "exit", || std::process::exit(0))
        .bind("1", "insert", insert)
        .bind("2", "remove", remove)
        .bind("3", "find", find)
        .bind("p", "print (debug)", print);

    loop {
        println!("Options:");
        input_buffer.prompt();
        input_buffer
            .read_input_and_call("Choice: ")
            .map_err(|e| println!("{}", e))
            .ok();
    }
}
