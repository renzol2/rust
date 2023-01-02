use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub name: String,
}

impl Student {
    // TODO: Implement this function
    /// Initialize a new student
    pub fn new(name: String) -> Student {
        todo!();
    }

    // TODO: Implement this function
    // Create a function that simulates the amount of time a student studies.
    // Print out two messages: "{student} is studying." and "{student} is done studying."
    // Between the two messages put the current thread to sleep for 1000 ms
    pub fn study(&self) {
        todo!();
    }
}

// TODO: Implement this function
// Iterate through the vector of students and pass them onto individual threads
// In each thread, call study();
// Return the handles
pub fn study_progress(students: Vec<Student>) -> Vec<JoinHandle<()>> {
    todo!();
}
