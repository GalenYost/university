pub mod heap;
pub mod input;

use heap::*;
use input::*;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let heap = Rc::new(RefCell::new(Heap::<String>::new()));

    let heap_push = heap.clone();
    let push = move || {
        let value = read_input_and_cast_value("Value: ", InputType::Text);
        let value = handle_value_cast_string(value);
        heap_push.borrow_mut().push(value);
    };

    let heap_pop = heap.clone();
    let pop = move || {
        if let Some(value) = heap_pop.borrow_mut().pop() {
            println!("dropped value: {}", value);
        }
    };

    let heap_swap = heap.clone();
    let swap = move || {
        heap_swap.borrow_mut().swap();
    };

    let heap_rev = heap.clone();
    let rev = move || {
        heap_rev.borrow_mut().reverse();
    };

    let heap_contains = heap.clone();
    let contains = move || {
        let value = read_input_and_cast_value("Value to check: ", InputType::Text);
        let value = handle_value_cast_string(value);
        println!("Result: {}", heap_contains.borrow().contains(&value));
    };

    let heap_clear = heap.clone();
    let clear = move || {
        heap_clear.borrow_mut().clear();
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
            println!("\nCurrent state: {}", heap.borrow())
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
