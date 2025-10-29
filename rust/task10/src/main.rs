pub mod input;
pub mod tree;

use input::*;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let bt = Rc::new(RefCell::new(BinaryTree::<usize>::new()));

    let insert_head_bt = bt.clone();
    let insert_head_cl = move || {
        let val: usize = read_input_and_cast_value("Value: ");
        insert_head_bt.borrow_mut().insert_head(val);
    };

    let insert_bt = bt.clone();
    let insert_cl = move || {
        let val: usize = read_input_and_cast_value("Value: ");
        let dir: PtrDirection = read_input_and_cast_value("Direction (LEFT/RIGHT): ");
        insert_bt.borrow_mut().push_under_ptr(val, dir);
    };

    let remove_bt = bt.clone();
    let remove_cl = move || {
        let dir: PtrDirection = read_input_and_cast_value("Direction (LEFT/RIGHT): ");
        remove_bt.borrow_mut().remove_under_ptr(dir);
    };

    let move_ptr_bt = bt.clone();
    let move_cl = move || {
        let dir: PtrDirection = read_input_and_cast_value("Direction (HEAD/UP/LEFT/RIGHT): ");
        move_ptr_bt.borrow_mut().move_ptr(dir);
    };

    let search_bt = bt.clone();
    let search_cl = move || {
        let val: usize = read_input_and_cast_value("Value to search: ");
        let order: SearchOrder = read_input_and_cast_value("Order (pre/in/post): ");
        let path: Vec<usize> = search_bt.borrow().search_for(val, order);
        if path.is_empty() {
            return;
        }
        println!("");
        path.iter().enumerate().for_each(|(i, el)| {
            if i + 1 == path.len() {
                print!("{}", el);
            } else {
                print!("{}->", el);
            }
        });
        println!("\n");
    };

    let display_bt = bt.clone();
    let display_cl = move || {
        println!("\n{}", display_bt.borrow());
    };

    let mut input_buffer = InputBuffer::new();
    input_buffer
        .bind("0", "exit", || std::process::exit(0))
        .bind("s", "swap head to new value", insert_head_cl)
        .bind("i", "insert new value under the pointer", insert_cl)
        .bind("r", "remove node under the pointer", remove_cl)
        .bind("m", "move the pointer", move_cl)
        .bind("f", "find the value and display the path", search_cl)
        .bind("d", "display the tree", display_cl);

    loop {
        println!("Options:");
        input_buffer.prompt();
        input_buffer
            .read_input_and_call("Choice: ")
            .map_err(|e| println!("{}", e))
            .ok();
    }
}
