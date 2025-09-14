#[cfg(test)]
pub mod tests;

pub mod input;
pub mod iter;
pub mod list;
pub mod student;

use input::*;
use list::*;
use student::*;

use std::process;
use std::sync::{Arc, Mutex};

fn main() {
    let lists = Arc::new(Mutex::new(Vec::<List>::new()));

    let lists_for_adding = Arc::clone(&lists);
    let add_first = move || {
        let mut locked = lists_for_adding.lock().unwrap();
        if locked.is_empty() {
            let info = prompt_student_info();
            let student = Student::new(&info.0, &info.1, &info.2, &info.3).rating(info.4);
            locked.push(List::new(student));
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if let Some(list) = locked.get_mut(list_idx) {
                let info = prompt_student_info();
                let student = Student::new(&info.0, &info.1, &info.2, &info.3).rating(info.4);
                list.insert_first(student.clone());
            }
        }
    };

    let lists_to_create = Arc::clone(&lists);
    let create_list = move || {
        let mut locked = lists_to_create.lock().unwrap();
        let info = prompt_student_info();
        let student = Student::new(&info.0, &info.1, &info.2, &info.3).rating(info.4);
        locked.push(List::new(student));
    };

    let lists_to_remove = Arc::clone(&lists);
    let remove_list = move || {
        let mut locked = lists_to_remove.lock().unwrap();
        if locked.is_empty() {
            let info = prompt_student_info();
            let student = Student::new(&info.0, &info.1, &info.2, &info.3).rating(info.4);
            locked.push(List::new(student));
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if locked.len() < list_idx {
                locked.remove(list_idx);
            } else {
                println!("index out of bounds");
            }
        }
    };

    let lists_to_insert_last = Arc::clone(&lists);
    let insert_last = move || {
        let mut locked = lists_to_insert_last.lock().unwrap();
        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if let Some(list) = locked.get_mut(list_idx) {
                let info = prompt_student_info();
                let student = Student::new(&info.0, &info.1, &info.2, &info.3).rating(info.4);
                list.insert_last(student.clone());
            }
        }
    };

    let lists_to_print = Arc::clone(&lists);
    let print_lists = move || {
        lists_to_print
            .lock()
            .unwrap()
            .iter()
            .for_each(|list| println!("list: {:?}", list));
    };

    let lists_to_insert_after_n = Arc::clone(&lists);
    let insert_after_n = move || {
        let mut locked = lists_to_insert_after_n.lock().unwrap();

        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if let Some(list) = locked.get_mut(list_idx) {
                let el_to_insert = prompt_student_info();
                let el_to_insert = Student::new(
                    &el_to_insert.0,
                    &el_to_insert.1,
                    &el_to_insert.2,
                    &el_to_insert.3,
                )
                .rating(el_to_insert.4);

                let insert_after = read_input_and_cast_value("n: ", InputType::Number);
                let insert_after = handle_value_cast_number(insert_after);

                list.insert_value_after_n(el_to_insert, insert_after);
            }
        }
    };

    let lists_to_move_after_n = Arc::clone(&lists);
    let move_after_n = move || {
        let mut locked = lists_to_move_after_n.lock().unwrap();

        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if let Some(list) = locked.get_mut(list_idx) {
                let el_to_move = read_input_and_cast_value("element index: ", InputType::Number);
                let el_to_move = handle_value_cast_number(el_to_move);

                let move_for = read_input_and_cast_value("for n: ", InputType::Number);
                let move_for = handle_value_cast_number(move_for);

                list.move_for_n(el_to_move, move_for);
            }
        }
    };

    let lists_to_remove_n = Arc::clone(&lists);
    let remove_n = move || {
        let mut locked = lists_to_remove_n.lock().unwrap();

        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if let Some(list) = locked.get_mut(list_idx) {
                let idx = read_input_and_cast_value("element index: ", InputType::Number);
                let idx = handle_value_cast_number(idx);

                list.remove_nth(idx);
            }
        }
    };

    let lists_to_remove_every_n = Arc::clone(&lists);
    let remove_every_n = move || {
        let mut locked = lists_to_remove_every_n.lock().unwrap();

        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if let Some(list) = locked.get_mut(list_idx) {
                let idx = read_input_and_cast_value("element index: ", InputType::Number);
                let idx = handle_value_cast_number(idx);

                list.remove_every_n(idx);
            }
        }
    };

    let lists_to_sort = Arc::clone(&lists);
    let sort_list = move || {
        let mut locked = lists_to_sort.lock().unwrap();

        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if let Some(list) = locked.get_mut(list_idx) {
                let sort_method = read_input_and_cast_value("sort method: ", InputType::Text);
                let sort_method = handle_value_cast_string(sort_method);
                let sort_method = match sort_method.trim().to_lowercase().as_str() {
                    "name" => SearchCriteria::Name("".into()),
                    "surname" => SearchCriteria::Surname("".into()),
                    "address" => SearchCriteria::Address("".into()),
                    "rating" => SearchCriteria::Rating(0),
                    _ => {
                        println!("no such option");
                        return;
                    }
                };

                let is_ascending_order_inp =
                    read_input_and_cast_value("ascending? (true or false): ", InputType::Text);
                let is_ascending_order_inp = handle_value_cast_string(is_ascending_order_inp);

                let is_ascending = match is_ascending_order_inp.trim().to_lowercase().as_str() {
                    "true" => true,
                    "false" => false,
                    _ => {
                        println!("wrong input");
                        return;
                    }
                };

                list.sort(sort_method, is_ascending);
            }
        }
    };

    let lists_to_copy = Arc::clone(&lists);
    let copy_list = move || {
        let mut locked = lists_to_copy.lock().unwrap();

        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if let Some(list) = locked.get_mut(list_idx) {
                let copied = list.clone();
                locked.push(copied);
            }
        }
    };

    let lists_to_clear = Arc::clone(&lists);
    let clear_list = move || {
        let mut locked = lists_to_clear.lock().unwrap();

        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx = read_input_and_cast_value("list index: ", InputType::Number);
            let list_idx = handle_value_cast_number(list_idx);

            if let Some(list) = locked.get_mut(list_idx) {
                list.clear();
            }
        }
    };

    let lists_to_merge = Arc::clone(&lists);
    let merge_lists = move || {
        let mut locked = lists_to_merge.lock().unwrap();

        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx1 = read_input_and_cast_value("list index 1: ", InputType::Number);
            let list_idx1 = handle_value_cast_number(list_idx1);

            let list_idx2 = read_input_and_cast_value("list index 2: ", InputType::Number);
            let list_idx2 = handle_value_cast_number(list_idx2);

            if list_idx1 < locked.len() && list_idx2 < locked.len() && list_idx1 != list_idx2 {
                let (i1, i2) = if list_idx1 < list_idx2 {
                    (list_idx1, list_idx2)
                } else {
                    (list_idx2, list_idx1)
                };

                let list2 = locked.remove(i2);
                locked[i1].append(list2);
            }
        }
    };

    let lists_to_intersect = Arc::clone(&lists);
    let intersect_lists = move || {
        let mut locked = lists_to_intersect.lock().unwrap();

        if locked.is_empty() {
            println!("no lists found in current scope");
            return;
        } else {
            let list_idx1 = read_input_and_cast_value("list index 1: ", InputType::Number);
            let list_idx1 = handle_value_cast_number(list_idx1);

            let list_idx2 = read_input_and_cast_value("list index 2: ", InputType::Number);
            let list_idx2 = handle_value_cast_number(list_idx2);

            if let Some(list) = locked.get(list_idx1)
                && let Some(list2) = locked.get(list_idx2)
            {
                if let Some(result_list) = List::intersection(&list, &list2) {
                    locked.push(result_list);
                } else {
                    println!("couldnt finish intersection list");
                }
            }
        }
    };

    let mut input_buffer = InputBuffer::new();
    input_buffer
        .bind("0", "exit", || process::exit(0))
        .bind("1", "add first", add_first)
        .bind("2", "insert after n", insert_after_n)
        .bind("3", "move after n", move_after_n)
        .bind("4", "remove n", remove_n)
        .bind("5", "remove every n", remove_every_n)
        .bind("6", "sort", sort_list)
        .bind("7", "copy list", copy_list)
        .bind("8", "clear list", clear_list)
        .bind("9", "merge lists", merge_lists)
        .bind("10", "intersectional list", intersect_lists)
        .bind("p", "print lists (debug tool)", print_lists)
        .bind("c", "create list (debug tool)", create_list)
        .bind("r", "remove list (debug tool)", remove_list)
        .bind("i", "insert last (debug tool)", insert_last);

    loop {
        println!("\nOptions:");
        input_buffer.prompt();
        input_buffer
            .read_input_and_call("Choice: ")
            .map_err(|e| println!("{}", e))
            .ok();
    }
}
