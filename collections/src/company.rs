use std::collections::HashMap;
use std::io;

// 3. Using a hash map and vectors, create a text interface to allow
// a user to add employee names to a department in a company. For
// example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
// let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.

// "Retrieve Engineering", "Retrieve Sales" -> retrieve all people in a department
// "Retrieve" -> retrieve all people in the company by department, sorted alphabetically

struct AddMessage {
    employee: String,
    department: String,
}

struct RetrieveMessage {
    by_department: bool,
    department: String,
}

enum Action {
    Add(AddMessage),
    Retrieve(RetrieveMessage),
    Quit,
    Help,
    Invalid(String),
}

/// Converts an input message to an `Action`
///
/// # Arguments
/// * `input` - an input given by user
fn get_action(input: &String) -> Action {
    let split_input: Vec<&str> = input.split(" ").collect();

    if split_input.len() == 0 {
        return Action::Invalid(String::from("No input detected."));
    }

    match split_input[0].trim() {
        "add" => {
            if split_input.len() < 3 {
                return Action::Invalid(String::from(
                    "'add' actions require an employee and department",
                ));
            }

            return Action::Add(AddMessage {
                employee: String::from(split_input[1]),
                department: String::from(split_input[2]),
            });
        }
        "retrieve" => {
            let by_department = split_input.len() > 1;
            let department = if by_department {
                String::from(split_input[1])
            } else {
                String::from("")
            };
            return Action::Retrieve(RetrieveMessage {
                by_department,
                department,
            });
        }
        "quit" => Action::Quit,
        "help" => Action::Help,
        _ => Action::Invalid(String::from("Invalid syntax. Please try again.")),
    }
}

fn print_help() {
    println!("Usage:");
    println!("- add [name] [department] --- adds employee to department");
    println!("- retrieve                --- retrieves all employees by department");
    println!("- retrieve [department]   --- retrieve employees of a specific department");
    println!("- quit                    --- exit program");
}

fn print_department(department: &String, employees: &Vec<String>) {
    println!("Employees in {}", department);
    for employee in employees {
        println!("- {}", employee);
    }
}

pub fn driver() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    print_help();

    loop {
        println!("");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let action = get_action(&input);

        // Process action
        match action {
            Action::Add(AddMessage {
                employee,
                department,
            }) => {
                let employees = departments.entry(department).or_insert(Vec::new());
                employees.push(employee);
            }
            Action::Retrieve(RetrieveMessage {
                by_department,
                department,
            }) => {
                if by_department {
                    match departments.get(&department) {
                        Option::Some(employees) => print_department(&department, employees),
                        Option::None => println!("No department named \"{}\" exists.", &department),
                    }
                } else {
                    for (department_key, employees) in departments.iter() {
                        print_department(department_key, employees);
                        println!("");
                    }
                }
            }
            Action::Quit => break,
            Action::Help => print_help(),
            Action::Invalid(message) => println!("{}", message),
        };
    }
}
