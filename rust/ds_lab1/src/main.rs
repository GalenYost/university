mod list;
mod input;

use std::rc::Rc;
use std::cell::RefCell;

use input::{InputBuffer, read_as};

fn main() {
    let list = Rc::new(RefCell::new(list::DoublyLinkedList::<i32>::new()));
    let mut menu = InputBuffer::new();

    menu.bind("e", "Exit", move || {
        std::process::exit(0);
    });

    let l = Rc::clone(&list);
    menu.bind("1", "Push front", move || {
        if let Ok(val) = read_as::<i32>("Enter int: ") {
            l.borrow_mut().push_front(val);
        }
    });

    let l = Rc::clone(&list);
    menu.bind("2", "Push back", move || {
        if let Ok(val) = read_as::<i32>("Enter int: ") {
            l.borrow_mut().push_back(val);
        }
    });

    let l = Rc::clone(&list);
    menu.bind("3", "Pop front", move || match l.borrow_mut().pop_front() {
        Some(val) => println!("Popped front: {val}"),
        None => println!("List is empty!"),
    });

    let l = Rc::clone(&list);
    menu.bind("4", "Pop back", move || match l.borrow_mut().pop_back() {
        Some(val) => println!("Popped back: {val}"),
        None => println!("List is empty!"),
    });

    let l = Rc::clone(&list);
    menu.bind("5", "Peek front", move || {
        let list_ref = l.borrow();
        println!("Front: {:?}", list_ref.front());
    });

    let l = Rc::clone(&list);
    menu.bind("6", "Peek back", move || {
        let list_ref = l.borrow();
        println!("Back: {:?}", list_ref.back());
    });

    let l = Rc::clone(&list);
    menu.bind("7", "Swap first & last", move || {
        l.borrow_mut().swap_first_last();
        println!("Swapped first and last elements.");
    });

    let l = Rc::clone(&list);
    menu.bind("8", "Contains value", move || {
        if let Ok(val) = read_as::<i32>("Value to search: ") {
            if l.borrow().contains(&val) {
                println!("Value {val} exists in the list.");
            } else {
                println!("Value {val} not found.");
            }
        }
    });

    let l = Rc::clone(&list);
    menu.bind("9", "Reverse list", move || {
        l.borrow_mut().reverse();
        println!("List reversed in place.");
    });

    let l = Rc::clone(&list);
    menu.bind("10", "Print list", move || {
        let list_ref = l.borrow();
        if list_ref.is_empty() {
            println!("List is empty.");
        } else {
            print!("List [len={}]: ", list_ref.len());
            for item in list_ref.iter() {
                print!("{item} ");
            }
            println!();
        }
    });

    let l = Rc::clone(&list);
    menu.bind("11", "Clear list", move || {
        l.borrow_mut().clear();
        println!("List cleared.");
    });

    let l = Rc::clone(&list);
    menu.bind("12", "Is empty", move || {
        println!("Is empty: {}", l.borrow_mut().is_empty());
    });

    while true {
        println!("\nOptions:");
        menu.prompt();

        if let Err(err) = menu.read_input_and_call("Selection: ") {
            println!("Error: {err}");
        }
    }
}
