pub mod input;
pub mod list;

use input::*;
use list::*;

fn main() {
    let mut input_buffer = InputBuffer::new();

    let task1 = || {
        let size = read_input_and_cast_value("Size of both teams: ", InputType::Number);
        let size = handle_value_cast_number(size);

        let team1: Vec<String> = (0..size)
            .enumerate()
            .map(|(i, _)| {
                let name = read_input_and_cast_value(
                    format!("(team 1) Player name #{}: ", i).as_str(),
                    InputType::Text,
                );
                handle_value_cast_string(name)
            })
            .collect();

        let team2: Vec<String> = (0..size)
            .enumerate()
            .map(|(i, _)| {
                let name = read_input_and_cast_value(
                    format!("(team 2) Player name #{}: ", i).as_str(),
                    InputType::Text,
                );
                handle_value_cast_string(name)
            })
            .collect();

        let list1 = List::<String>::new(Some(team1));
        let list2 = List::<String>::new(Some(team2));

        let n = read_input_and_cast_value("n: ", InputType::Number);
        let n = handle_value_cast_number(n);

        let m = read_input_and_cast_value("m: ", InputType::Number);
        let m = handle_value_cast_number(m);

        let new_list1 = list1.get_every_nth(n);
        let new_list2 = list2.get_every_nth(m);

        println!("Team 1: ");
        new_list1.iter().for_each(|el| {
            println!("{}", el);
        });

        println!("Team 2: ");
        new_list2.iter().for_each(|el| {
            println!("{}", el);
        });
    };

    let task2 = || {
        let n = read_input_and_cast_value("n: ", InputType::Number);
        let n = handle_value_cast_number(n);

        let m = read_input_and_cast_value("m: ", InputType::Number);
        let m = handle_value_cast_number(m);

        let soldiers: Vec<String> = (1..=n)
            .enumerate()
            .map(|(i, _)| {
                let name = read_input_and_cast_value(
                    format!("Soldier #{}: ", i).as_str(),
                    InputType::Text,
                );
                handle_value_cast_string(name)
            })
            .collect();

        let mut list = List::<String>::new(Some(soldiers));
        let mut idx = 0;

        while list.len() > 1 {
            idx = (idx + m - 1) % list.len();
            list.remove_nth(idx);
        }

        println!("last: {}", list.get_el_n(0).unwrap());
    };

    let task3 = || {
        let size_of_people = read_input_and_cast_value("Amount of people: ", InputType::Number);
        let size_of_people = handle_value_cast_number(size_of_people);

        let prizes_amount = read_input_and_cast_value("Amount of prizes: ", InputType::Number);
        let prizes_amount = handle_value_cast_number(prizes_amount);

        let people: Vec<String> = (0..size_of_people)
            .enumerate()
            .map(|(i, _)| {
                let name =
                    read_input_and_cast_value(format!("Person #{}: ", i).as_str(), InputType::Text);
                handle_value_cast_string(name)
            })
            .collect();
        let people = List::<String>::new(Some(people));

        let prizes: Vec<String> = (0..prizes_amount)
            .enumerate()
            .map(|(i, _)| {
                let name =
                    read_input_and_cast_value(format!("Prize #{}: ", i).as_str(), InputType::Text);
                handle_value_cast_string(name)
            })
            .collect();
        let prizes = List::<String>::new(Some(prizes));

        let n = read_input_and_cast_value("Amount of winners (n): ", InputType::Number);
        let n = handle_value_cast_number(n);

        let k = read_input_and_cast_value("Every k-th winner (k): ", InputType::Number);
        let k = handle_value_cast_number(k);

        let t = read_input_and_cast_value("Prize recount (t): ", InputType::Number);
        let t = handle_value_cast_number(t);

        let mut p_idx = 0;
        let mut pr_idx = 0;

        for _ in 0..n {
            p_idx = (p_idx + k - 1) % size_of_people;
            pr_idx = (pr_idx + t - 1) % prizes_amount;

            let person = people.get_el_n(p_idx).unwrap();
            let prize = prizes.get_el_n(pr_idx).unwrap();

            println!("Winner: {}", person);
            println!("Prize: {}", prize);

            p_idx = (p_idx + 1) % size_of_people;
        }
    };

    let task4 = || {
        let size = read_input_and_cast_value("List size: ", InputType::Number);
        let size = handle_value_cast_number(size);

        let students: Vec<String> = (0..size)
            .enumerate()
            .map(|(i, _)| {
                let name = read_input_and_cast_value(
                    format!("Student #{}: ", i).as_str(),
                    InputType::Text,
                );
                handle_value_cast_string(name)
            })
            .collect();
        let students = List::<String>::new(Some(students));

        let tickets: Vec<String> = (0..size)
            .enumerate()
            .map(|(i, _)| {
                let name =
                    read_input_and_cast_value(format!("Ticket #{}: ", i).as_str(), InputType::Text);
                handle_value_cast_string(name)
            })
            .collect();
        let tickets = List::<String>::new(Some(tickets));

        let n = read_input_and_cast_value("n: ", InputType::Number);
        let n = handle_value_cast_number(n);

        let k = read_input_and_cast_value("k: ", InputType::Number);
        let k = handle_value_cast_number(k);

        let mut st_idx = 0;
        let mut tk_idx = 0;

        for _ in 0..size {
            st_idx = (st_idx + k - 1) % size;
            tk_idx = (tk_idx + n - 1) % size;

            println!("Student: {}", students.get_el_n(st_idx).unwrap());
            println!("Ticket: {}", tickets.get_el_n(tk_idx).unwrap());
        }
    };

    let task5 = || {
        let t1 = read_input_and_cast_value("Product amount: ", InputType::Number);
        let t1 = handle_value_cast_number(t1);

        let t2 = read_input_and_cast_value("Customer amount: ", InputType::Number);
        let t2 = handle_value_cast_number(t2);

        let products: Vec<String> = (0..t1)
            .enumerate()
            .map(|(i, _)| {
                let name = read_input_and_cast_value(
                    format!("Product #{}: ", i).as_str(),
                    InputType::Text,
                );
                handle_value_cast_string(name)
            })
            .collect();
        let products = List::<String>::new(Some(products));

        let customers: Vec<String> = (0..t2)
            .enumerate()
            .map(|(i, _)| {
                let name = read_input_and_cast_value(
                    format!("Customer #{}: ", i).as_str(),
                    InputType::Text,
                );
                handle_value_cast_string(name)
            })
            .collect();
        let customers = List::<String>::new(Some(customers));

        let n = read_input_and_cast_value("n: ", InputType::Number);
        let n = handle_value_cast_number(n);

        let m = read_input_and_cast_value("m: ", InputType::Number);
        let m = handle_value_cast_number(m);

        let mut p_idx = 0;
        let mut c_idx = 0;

        let pairs = std::cmp::min(t1, t2);

        for _ in 0..pairs {
            p_idx = (p_idx + n - 1) % t1;
            c_idx = (c_idx + m - 1) % t2;

            println!("Customer: {}", customers.get_el_n(c_idx).unwrap());
            println!("Product: {}", products.get_el_n(p_idx).unwrap());
        }
    };

    input_buffer
        .bind("0", "exit", || std::process::exit(0))
        .bind("1", "1.1", task1)
        .bind("2", "1.2", task2)
        .bind("3", "1.3", task3)
        .bind("4", "1.4", task4)
        .bind("5", "1.5", task5);

    loop {
        println!("\nOptions:");
        input_buffer.prompt();
        input_buffer
            .read_input_and_call("Choice: ")
            .map_err(|e| println!("{}", e))
            .ok();
    }
}
