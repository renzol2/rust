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
    Invalid,
}

/// Converts an input message to an `Action`
///
/// # Arguments
/// * `input` - an input given by user
fn get_action(input: &String) -> Action {
    Action::Invalid
}

/// Adds an employee to the departments map.
/// Handles duplicate employees gracefully.
///
/// # Arguments
/// * `msg` - the message containing the employee's name and address
/// * `departments` - the current departments and lists of employees
fn add_employee(msg: &AddMessage, departments: &HashMap<String, Vec<String>>) {}

pub fn driver() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        // Add directions for user

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let action = get_action(&input);

        // Process action

        // Add break condition
    }
}
