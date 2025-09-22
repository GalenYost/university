use std::collections::LinkedList;

pub mod input;
use input::*;

fn remove_nth(list: &mut LinkedList<String>, n: usize) -> Option<String> {
    if n >= list.len() {
        return None;
    }
    let mut split = list.split_off(n);
    let result = split.pop_front();
    list.append(&mut split);
    result
}

fn main() {
    let mut input_buffer = InputBuffer::new();

    let task1 = || {
        let size = read_input_and_cast_value("Size of both teams: ", InputType::Number);
        let size = handle_value_cast_number(size);

        let mut team1 = LinkedList::<String>::new();
        let mut team2 = LinkedList::<String>::new();

        (0..size).enumerate().for_each(|(i, _)| {
            let name = read_input_and_cast_value(
                format!("(team 1) Player name #{}: ", i).as_str(),
                InputType::Text,
            );
            let name = handle_value_cast_string(name);
            team1.push_back(name);
        });

        (0..size).enumerate().for_each(|(i, _)| {
            let name = read_input_and_cast_value(
                format!("(team 2) Player name #{}: ", i).as_str(),
                InputType::Text,
            );
            let name = handle_value_cast_string(name);
            team2.push_back(name);
        });

        let n = read_input_and_cast_value("n: ", InputType::Number);
        let n = handle_value_cast_number(n);

        let m = read_input_and_cast_value("m: ", InputType::Number);
        let m = handle_value_cast_number(m);

        let result1: Vec<_> = team1
            .iter()
            .enumerate()
            .filter_map(|(i, val)| if i % n == n - 1 { Some(val) } else { None })
            .collect();
        let result2: Vec<_> = team2
            .iter()
            .enumerate()
            .filter_map(|(i, val)| if i % m == m - 1 { Some(val) } else { None })
            .collect();

        println!("Team 1: ");
        result1.iter().for_each(|el| {
            println!("{}", el);
        });

        println!("Team 2: ");
        result2.iter().for_each(|el| {
            println!("{}", el);
        });
    };

    let task2 = || {
        let n = read_input_and_cast_value("n: ", InputType::Number);
        let n = handle_value_cast_number(n);

        let m = read_input_and_cast_value("m: ", InputType::Number);
        let m = handle_value_cast_number(m);

        let mut list = LinkedList::<String>::new();

        (0..n).enumerate().for_each(|(i, _)| {
            let name =
                read_input_and_cast_value(format!("Soldier #{}: ", i).as_str(), InputType::Text);
            let name = handle_value_cast_string(name);
            list.push_back(name);
        });

        let mut idx = 0;
        while list.len() > 1 {
            idx = (idx + m - 1) % list.len();
            remove_nth(&mut list, idx);
        }

        println!("last: {}", list.front().unwrap());
    };

    let task3 = || {
        let size_of_people = read_input_and_cast_value("Amount of people: ", InputType::Number);
        let size_of_people = handle_value_cast_number(size_of_people);

        let prizes_amount = read_input_and_cast_value("Amount of prizes: ", InputType::Number);
        let prizes_amount = handle_value_cast_number(prizes_amount);

        let mut people = LinkedList::<String>::new();
        let mut prizes = LinkedList::<String>::new();

        (0..size_of_people).enumerate().for_each(|(i, _)| {
            let name =
                read_input_and_cast_value(format!("Person #{}: ", i).as_str(), InputType::Text);
            let name = handle_value_cast_string(name);
            people.push_back(name);
        });

        (0..prizes_amount).enumerate().for_each(|(i, _)| {
            let name =
                read_input_and_cast_value(format!("Prize #{}: ", i).as_str(), InputType::Text);
            let name = handle_value_cast_string(name);
            prizes.push_back(name);
        });

        let n = read_input_and_cast_value("Amount of winners (n): ", InputType::Number);
        let n = handle_value_cast_number(n);

        let k = read_input_and_cast_value("Every k-th winner (k): ", InputType::Number);
        let k = handle_value_cast_number(k);

        let t = read_input_and_cast_value("Prize recount (t): ", InputType::Number);
        let t = handle_value_cast_number(t);

        let mut p_idx = 0;
        let mut pr_idx = 0;

        for i in 0..n {
            p_idx = (p_idx + k - 1) % people.len();

            if t == 0 {
                pr_idx = i % prizes.len();
            } else {
                pr_idx = (pr_idx + t - 1) % prizes.len();
            }

            let winner = remove_nth(&mut people, p_idx).unwrap();
            let prize = remove_nth(&mut prizes, pr_idx).unwrap();

            println!("Winner: {}, Prize: {}", winner, prize);

            p_idx = (p_idx + 1) % people.len();
        }
    };

    let task4 = || {
        let size = read_input_and_cast_value("List size: ", InputType::Number);
        let size = handle_value_cast_number(size);

        let mut students = LinkedList::<String>::new();
        let mut tickets = LinkedList::<String>::new();

        (0..size).enumerate().for_each(|(i, _)| {
            let name =
                read_input_and_cast_value(format!("Student #{}: ", i).as_str(), InputType::Text);
            let name = handle_value_cast_string(name);
            students.push_back(name);
        });

        (0..size).enumerate().for_each(|(i, _)| {
            let name =
                read_input_and_cast_value(format!("Ticket #{}: ", i).as_str(), InputType::Text);
            let name = handle_value_cast_string(name);
            tickets.push_back(name);
        });

        let n = read_input_and_cast_value("n: ", InputType::Number);
        let n = handle_value_cast_number(n);

        let k = read_input_and_cast_value("k: ", InputType::Number);
        let k = handle_value_cast_number(k);

        let mut st_idx = 0;
        let mut tk_idx = 0;

        for _ in 0..size {
            st_idx = (st_idx + k - 1) % students.len();
            tk_idx = (tk_idx + n - 1) % tickets.len();

            let student = remove_nth(&mut students, st_idx).unwrap();
            let ticket = remove_nth(&mut tickets, tk_idx).unwrap();

            println!("Student: {}, Ticket: {}", student, ticket);
        }
    };

    let task5 = || {
        let t1 = read_input_and_cast_value("Product amount: ", InputType::Number);
        let t1 = handle_value_cast_number(t1);

        let t2 = read_input_and_cast_value("Customer amount: ", InputType::Number);
        let t2 = handle_value_cast_number(t2);

        let mut products = LinkedList::<String>::new();
        let mut customers = LinkedList::<String>::new();

        (0..t1).enumerate().for_each(|(i, _)| {
            let name =
                read_input_and_cast_value(format!("Product #{}: ", i).as_str(), InputType::Text);
            let name = handle_value_cast_string(name);
            products.push_back(name);
        });

        (0..t2).enumerate().for_each(|(i, _)| {
            let name =
                read_input_and_cast_value(format!("Customer #{}: ", i).as_str(), InputType::Text);
            let name = handle_value_cast_string(name);
            customers.push_back(name);
        });

        let n = read_input_and_cast_value("n: ", InputType::Number);
        let n = handle_value_cast_number(n);

        let m = read_input_and_cast_value("m: ", InputType::Number);
        let m = handle_value_cast_number(m);

        let mut p_idx = 0;
        let mut c_idx = 0;

        let pairs = std::cmp::min(t1, t2);

        for _ in 0..pairs {
            p_idx = (p_idx + n - 1) % products.len();
            c_idx = (c_idx + m - 1) % customers.len();

            let customer = remove_nth(&mut customers, c_idx).unwrap();
            let product = remove_nth(&mut products, p_idx).unwrap();

            println!("Customer: {}, Product: {}", customer, product);
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
