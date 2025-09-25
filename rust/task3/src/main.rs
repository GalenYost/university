pub mod input;
pub mod stack;

use input::*;
use stack::*;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let stack = Rc::new(RefCell::new(Stack::<String>::new()));

    let stack_push = stack.clone();
    let push = move || {
        let value = read_input_and_cast_value("Value: ", InputType::Text);
        let value = handle_value_cast_string(value);
        stack_push.borrow_mut().push(value);
    };

    let stack_pop = stack.clone();
    let pop = move || {
        if let Some(value) = stack_pop.borrow_mut().pop() {
            println!("dropped value: {}", value);
        }
    };

    let stack_swap = stack.clone();
    let swap = move || {
        stack_swap.borrow_mut().swap();
    };

    let stack_rev = stack.clone();
    let rev = move || {
        stack_rev.borrow_mut().reverse();
    };

    let stack_contains = stack.clone();
    let contains = move || {
        let value = read_input_and_cast_value("Value to check: ", InputType::Text);
        let value = handle_value_cast_string(value);
        println!("Result: {}", stack_contains.borrow().contains(&value));
    };

    let stack_clear = stack.clone();
    let clear = move || {
        stack_clear.borrow_mut().clear();
    };

    let mut input_buffer = InputBuffer::new();
    input_buffer
        .bind("0", "exit", || std::process::exit(0))
        .bind("1", "push", push)
        .bind("2", "pop", pop)
        .bind("3", "swap", swap)
        .bind("4", "reverse", rev)
        .bind("5", "contains", contains)
        .bind("6", "clear", clear)
        .bind("p", "print (debug tool)", move || {
            println!("\nCurrent state: {}", stack.borrow())
        });

    loop {
        println!("\nOptions:");
        input_buffer.prompt();
        input_buffer
            .read_input_and_call("Choice: ")
            .map_err(|e| println!("{}", e))
            .ok();
    }
}
