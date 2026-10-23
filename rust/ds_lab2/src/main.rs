mod input;
mod list;

use input::{InputBuffer, read_as, read_until};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn max_level_mode_log(p: f64, len: usize) -> usize {
    if len <= 1 {
        return 1;
    }

    let base = if p > 0.0 && p < 1.0 {
        1.0 / p
    } else {
        std::process::exit(0);
    };

    let max_lvl = (len as f64).log(base).ceil() as usize;
    max_lvl.max(1)
}

fn create_list() -> list::SkipList<i32, i32> {
    println!("------------");
    println!("Format:\n- deterministic/d\n- random/r");
    let list_level_mode = read_until::<list::LevelMode>("Mode: ");

    println!("------------");
    println!("Format:\n- constant(value)/c(value)\n- unlimited/u\n- log/l");
    let max_level_mode = read_until::<list::MaxLevelMode>("Max Level Mode: ");

    let probability = if list_level_mode == list::LevelMode::Random {
        println!("------------");
        println!("Format:\n0.98\n0 < p < 1");
        read_until::<f64>("Probability: ")
    } else { 0.0 };

    list::SkipList::<i32, i32>::new(list_level_mode, max_level_mode, probability)
}

fn create_menu(list: Rc<RefCell<list::SkipList<i32, i32>>>) -> InputBuffer {
    let mut menu = InputBuffer::new();

    menu.bind("e", "Exit", move || {
        std::process::exit(0);
    });

    let print_list = Rc::clone(&list);
    menu.bind("1", "print list", move || {
        print!("{}", print_list.borrow());
    });

    let insert_list = Rc::clone(&list);
    menu.bind("2", "insert", move || {
        let key = read_until::<i32>("Key (i32): ");
        let value = read_until::<i32>("Value (i32): ");
        insert_list.borrow_mut().insert(key, value);
    });

    let search_list = Rc::clone(&list);
    menu.bind("3", "search", move || {
        let key = read_until::<i32>("Key (i32): ");
        if let Some(val) = search_list.borrow().search(&key) {
            println!("Some({})", val);
        } else {
            println!("None");
        }
    });

    let delete_list = Rc::clone(&list);
    menu.bind("4", "delete", move || {
        let key = read_until::<i32>("Key (i32): ");
        delete_list.borrow_mut().delete(&key);
    });

    let copy_list = Rc::clone(&list);
    menu.bind("5", "copy", move || {
        println!("=== Copy list ===");
        print!("{}", copy_list.borrow().copy());
        println!("=== ========= ===");
    });

    let clear_list = Rc::clone(&list);
    menu.bind("6", "clear", move || {
        clear_list.borrow_mut().clear();
    });

    menu.bind("r", "replace list", move || {
        let new_list = create_list();
        *list.borrow_mut() = new_list;
    });

    menu
}

fn main() {
    let list = Rc::new(RefCell::new(create_list()));
    let mut menu = create_menu(Rc::clone(&list));

    while true {
        println!("\nOptions:");
        menu.prompt();

        if let Err(err) = menu.read_input_and_call("Selection: ") {
            println!("Error: {err}");
        }
    }
}
