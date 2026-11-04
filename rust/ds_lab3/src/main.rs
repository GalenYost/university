mod input;
mod matrix;

use input::{read_as, read_until, InputBuffer};
use matrix::Matrix;
use std::cell::RefCell;
use std::rc::Rc;

struct NamedMatrix {
    name: String,
    matrix: Matrix<i32>,
}

fn print_matrices_summary(matrices: &[NamedMatrix]) {
    if matrices.is_empty() {
        println!("Empty");
        return;
    }
    for (i, m) in matrices.iter().enumerate() {
        println!(
            "  [{}] \"{}\" : {}x{}, non-zero elements: {}",
            i,
            m.name,
            m.matrix.rows(),
            m.matrix.cols(),
            m.matrix.non_zero_count()
        );
    }
}

fn select_matrix_index(matrices: &[NamedMatrix], prompt: &str) -> Option<usize> {
    if matrices.is_empty() {
        println!("Error: Create at least one matrix");
        return None;
    }
    print_matrices_summary(matrices);
    let idx = read_until::<usize>(prompt);
    if idx < matrices.len() {
        Some(idx)
    } else {
        println!(
            "Error: {} is out of bounds (available: 0..{})",
            idx,
            matrices.len().saturating_sub(1)
        );
        None
    }
}

fn read_name(prompt: &str) -> String {
    loop {
        match read_as::<String>(prompt) {
            Ok(s) if !s.trim().is_empty() => return s.trim().to_string(),
            Ok(_) => println!("Error: cannot be empty!"),
            Err(e) => println!("Input error: {e}"),
        }
    }
}

fn create_menu(matrices: Rc<RefCell<Vec<NamedMatrix>>>) -> InputBuffer {
    let mut menu = InputBuffer::new();

    {
        let mats = Rc::clone(&matrices);
        menu.bind("1", "print matrix", move || {
            let borrow = mats.borrow();
            if let Some(idx) = select_matrix_index(&borrow, "Index: ") {
                println!("\n--- Matrix [{idx}] \"{}\" ---", borrow[idx].name);
                print!("{}", borrow[idx].matrix);
            }
        });
    }

    {
        let mats = Rc::clone(&matrices);
        menu.bind("2", "create matrix", move || {
            let name = read_name("Name: ");
            let rows = read_until::<usize>("Rows: ");
            let cols = read_until::<usize>("Columns: ");

            let mut borrow = mats.borrow_mut();
            let idx = borrow.len();
            borrow.push(NamedMatrix {
                name: name.clone(),
                matrix: Matrix::new(rows, cols),
            });
            println!("Matrix \"{name}\" ({rows}x{cols}) created. [{idx}].");
        });
    }

    {
        let mats = Rc::clone(&matrices);
        menu.bind("3", "set", move || {
            let idx = {
                let borrow = mats.borrow();
                select_matrix_index(&borrow, "Index: ")
            };

            if let Some(idx) = idx {
                let row = read_until::<usize>("Row (0-based): ");
                let col = read_until::<usize>("Column (0-based): ");
                let val = read_until::<i32>("Value: ");

                let mut borrow = mats.borrow_mut();
                borrow[idx].matrix.set(row, col, val);
                println!(
                    "Element ({row}, {col}) = {val} in \"{}\" successfully updated",
                    borrow[idx].name
                );
                println!("\nNew matrix:");
                print!("{}", borrow[idx].matrix);
            }
        });
    }

    {
        let mats = Rc::clone(&matrices);
        menu.bind("4", "multiply", move || {
            let idx = {
                let borrow = mats.borrow();
                select_matrix_index(&borrow, "Index: ")
            };
            if let Some(idx) = idx {
                let scalar = read_until::<i32>("Scalar: ");
                let new_name = read_name("Name: ");

                let mut borrow = mats.borrow_mut();
                let result_matrix = borrow[idx].matrix.mul_scalar(scalar);
                let new_idx = borrow.len();
                borrow.push(NamedMatrix {
                    name: new_name.clone(),
                    matrix: result_matrix,
                });

                println!(
                    "\nCreated new matrix [{new_idx}] \"{new_name}\" = [{idx}] \"{}\" * {scalar}:",
                    borrow[idx].name
                );
                print!("{}", borrow[new_idx].matrix);
            }
        });
    }

    {
        let mats = Rc::clone(&matrices);
        menu.bind("5", "transpose matrix", move || {
            let idx = {
                let borrow = mats.borrow();
                select_matrix_index(&borrow, "Index: ")
            };
            if let Some(idx) = idx {
                let new_name = read_name("Name: ");

                let mut borrow = mats.borrow_mut();
                let result_matrix = borrow[idx].matrix.transpose();
                let new_idx = borrow.len();
                borrow.push(NamedMatrix {
                    name: new_name.clone(),
                    matrix: result_matrix,
                });

                println!(
                    "\nCreated new matrix [{new_idx}] \"{new_name}\" with [{idx}] \"{}\":",
                    borrow[idx].name
                );
                print!("{}", borrow[new_idx].matrix);
            }
        });
    }

    {
        let mats = Rc::clone(&matrices);
        menu.bind("6", "sum matrices", move || {
            let (idx_a, idx_b) = {
                let borrow = mats.borrow();
                let a = select_matrix_index(&borrow, "Matrix A index: ");
                if a.is_none() {
                    return;
                }
                let b = select_matrix_index(&borrow, "Matrix B index: ");
                if b.is_none() {
                    return;
                }
                (a.unwrap(), b.unwrap())
            };

            let mut borrow = mats.borrow_mut();
            match borrow[idx_a].matrix.add(&borrow[idx_b].matrix) {
                Ok(result_matrix) => {
                    let new_name =
                        read_name("Name for matrix C: ");
                    let new_idx = borrow.len();
                    borrow.push(NamedMatrix {
                        name: new_name.clone(),
                        matrix: result_matrix,
                    });
                    println!(
                        "\nCreated new matrix [{new_idx}] \"{new_name}\" = [{idx_a}] + [{idx_b}]:"
                    );
                    print!("{}", borrow[new_idx].matrix);
                }
                Err(err) => {
                    println!("\nError: {err}");
                }
            }
        });
    }

    {
        let mats = Rc::clone(&matrices);
        menu.bind("7", "print list", move || {
            print_matrices_summary(&mats.borrow());
        });
    }


    {
        let mats = Rc::clone(&matrices);
        menu.bind("8", "remove", move || {
            let mut borrow = mats.borrow_mut();
            if let Some(idx) = select_matrix_index(&borrow, "Index: ") {
                let removed = borrow.remove(idx);
                println!("Matrix [{idx}] \"{}\" deleted", removed.name);
            }
        });
    }

    menu.bind("e", "exit", move || {
        std::process::exit(0);
    });

    menu
}

fn main() {
    let initial_matrices = vec![];
    let matrices = Rc::new(RefCell::new(initial_matrices));
    let mut menu = create_menu(Rc::clone(&matrices));

    loop {
        println!("\nOptions:");
        menu.prompt();

        if let Err(err) = menu.read_input_and_call("\nSelect: ") {
            println!("Error: {err}");
        }
    }
}
