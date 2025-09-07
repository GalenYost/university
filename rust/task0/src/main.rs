#[cfg(test)]
pub mod tests;

pub mod input;
pub mod student;

use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use input::InputBuffer;
use student::{Group, Student, prompt_group_code, prompt_search_criteria, prompt_student_info};

fn main() {
    let mut input_buffer = InputBuffer::new();
    let groups = Rc::new(RefCell::new(Vec::<Group>::new()));

    let groups_create = Rc::clone(&groups);
    let create_group = move || {
        let group_code = prompt_group_code();
        groups_create.borrow_mut().push(Group::new(&group_code));
    };

    let groups_list = Rc::clone(&groups);
    let list_groups = move || {
        for group in groups_list.borrow().iter() {
            group.display_group();
        }
    };

    let add_to_group_list = Rc::clone(&groups);
    let add_to_group = move || {
        let group_code = prompt_group_code();
        let group_exists = {
            let groups_ref = add_to_group_list.borrow();
            groups_ref.iter().any(|g| g.get_code() == group_code)
        };

        if group_exists {
            let (name, surname, address, rating) = prompt_student_info();

            let mut groups_ref = add_to_group_list.borrow_mut();
            if let Some(g) = groups_ref.iter_mut().find(|g| g.get_code() == group_code) {
                g.add(Student::new(&name, &surname, &address).rating(rating));
            }
        } else {
            println!("group not found: {}", group_code);
        }
    };

    let remove_from_group_list = Rc::clone(&groups);
    let remove_from_group = move || {
        let group_code = prompt_group_code();
        let group_exists = {
            let groups_ref = remove_from_group_list.borrow();
            groups_ref.iter().any(|g| g.get_code() == group_code)
        };

        if group_exists {
            let mut groups_ref = remove_from_group_list.borrow_mut();
            if let Some(g) = groups_ref.iter_mut().find(|g| g.get_code() == group_code) {
                let criteria = prompt_search_criteria();
                g.remove(criteria);
            }
        } else {
            println!("group not found: {}", group_code);
        }
    };

    let display_student_list = Rc::clone(&groups);
    let display_student = move || {
        let group_code = prompt_group_code();
        let group_exists = {
            let groups_ref = display_student_list.borrow();
            groups_ref.iter().any(|g| g.get_code() == group_code)
        };

        if group_exists {
            let groups_ref = display_student_list.borrow();
            if let Some(g) = groups_ref.iter().find(|g| g.get_code() == group_code) {
                let criteria = prompt_search_criteria();
                g.display_student(criteria);
            }
        } else {
            println!("group not found: {}", group_code);
        }
    };

    let display_group_list = Rc::clone(&groups);
    let display_group = move || {
        let group_code = prompt_group_code();
        let group_exists = {
            let groups_ref = display_group_list.borrow();
            groups_ref.iter().any(|g| g.get_code() == group_code)
        };

        if group_exists {
            let groups_ref = display_group_list.borrow();
            if let Some(g) = groups_ref.iter().find(|g| g.get_code() == group_code) {
                g.display_group();
            }
        } else {
            println!("group not found: {}", group_code);
        }
    };

    input_buffer
        .bind("0", "exit", || process::exit(0))
        .bind("1", "create group", create_group)
        .bind("2", "list group", list_groups)
        .bind("3", "add student to group", add_to_group)
        .bind("4", "remove student from group", remove_from_group)
        .bind("5", "display student", display_student)
        .bind("6", "display group", display_group);

    loop {
        println!("\nOptions:");
        input_buffer.prompt();
        input_buffer
            .read_input_and_call("Choice: ")
            .map_err(|e| println!("{}", e))
            .ok();
    }
}
