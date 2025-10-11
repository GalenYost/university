pub mod input;
pub mod tree;

use input::*;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut input_buffer = InputBuffer::new();
    input_buffer.bind("0", "exit", || std::process::exit(0));

    loop {
        println!("Options:");
        input_buffer.prompt();
        input_buffer
            .read_input_and_call("Choice: ")
            .map_err(|e| println!("{}", e))
            .ok();
    }
}
