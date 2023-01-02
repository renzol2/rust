/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub name: String,
    pub netid: String,
    pub schedule: Schedule,
}

/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub credit_hours: u8,
}

/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug, Clone, PartialEq)]
pub struct Schedule {
    pub classes: Vec<Class>,
    pub credit_hours: u8,
}

impl Student {
    // TODO: Implement this function
    /// Initialize a new student
    /// Set the students schedule with a new empty schedule
    pub fn new(name: String, netid: String) -> Student {
        todo!();
    }

    // TODO: Implement this function
    /// Enroll a student in a schedule
    pub fn schedule_enrollment(&mut self, schedule: Schedule) {
        todo!();
    }

    // TODO: Implement this function
    /// Check if a student is a classmate of another student
    pub fn is_classmate(&self, other: &Student) -> bool {
        todo!();
    }
}

impl Class {
    // TODO: Implement this function
    /// Initialize a new class
    pub fn new(name: String, credit_hours: u8) -> Class {
        todo!();
    }
}

impl Schedule {
    // TODO: Implement this function
    /// Initialize a new schedule
    /// Credit hours should be total up from the classes
    pub fn new(classes: Vec<Class>) -> Schedule {
        todo!();
    }
}