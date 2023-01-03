/// This struct represents a student at UIUC
/// It stores their name and netid
/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub name: String,
    pub netid: String,
}

/// This struct represents a class at UIUC
/// It stores the name of the class and the list of students enrolled
/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub attendance: Vec<Student>,
}

/// This function initializes a student with the given name and netid
pub fn student_init(name: String, netid: String) -> Student {
    Student {
        name: name,
        netid: netid,
    }
}

/// This function initializes a class with the given name and an empty list of students
/// It should set the class name to the given String and the list of students to a new empty vector
pub fn class_init(name: String) -> Class {
    Class {
        name,
        attendance: Vec::new(),
    }
}

/// This function returns information about the given student
/// It returns a tuple of the student's name and netid
pub fn student_info(student: &Student) -> (String, String) {
    (student.name.clone(), student.netid.clone())
}

/// This function returns information about the given class
/// It returns the name of the class and the number of students enrolled
pub fn class_info(class: &Class) -> (String, usize) {
    (class.name.clone(), class.attendance.len())
}

/// This function enrolls the given student in the given class
/// It then returns the full attendance list for the class
pub fn class_enrollment(student: &Student, class: &mut Class) -> Vec<Student> {
    class.attendance.push(student.clone());
    class.attendance.clone()
}

/// This function returns some vector of the given students
/// and whether they are enrolled in the given class
/// If their are no students enrolled then return None
pub fn get_attendance(students: Vec<&Student>, class: &Class) -> Option<Vec<(Student, bool)>> {
    let mut no_students = true;
    let attendance: Vec<(Student, bool)> = students
        .iter()
        .map(|s| match class.attendance.contains(*s) {
            true => {
                no_students = false;
                return ((*s).clone(), true);
            }
            false => ((*s).clone(), false),
        })
        .collect();

    if no_students {
        None
    } else {
        Some(attendance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_init_creates_class_correctly() {
        let cs_128 = class_init("CS 128".to_string());
        assert_eq!(cs_128.name, "CS 128".to_string());
        assert_eq!(cs_128.attendance, Vec::new());
    }

    #[test]
    fn class_info_gets_info_correctly() {
        let class = class_init("CS 128".to_string());
        let (name, size) = class_info(&class);
        assert_eq!(name, "CS 128".to_string());
        assert_eq!(size, 0);
    }

    #[test]
    fn class_enrollment_enrolls_student_correctly() {
        let mut class = class_init("CS 128".to_string());
        let (name, size) = class_info(&class);
        assert_eq!(name, "CS 128".to_string());
        assert_eq!(size, 0);

        let student = student_init("Dev".to_string(), "dev".to_string());
        let attendance = class_enrollment(&student, &mut class);
        assert_eq!(class.attendance.len(), 1);
        assert_eq!(attendance.len(), 1);
        assert_eq!(attendance.first().unwrap(), &student);

        let student2 = student_init("Foo".to_string(), "foo".to_string());
        let attendance = class_enrollment(&student2, &mut class);
        assert_eq!(class.attendance.len(), 2);
        assert_eq!(attendance.len(), 2);
        assert_eq!(attendance.first().unwrap(), &student);
        assert_eq!(attendance.get(1).unwrap(), &student2);
    }

    #[test]
    fn get_attendance_works_correctly() {
        let mut class = class_init("MUS 499".to_string());
        let (name, size) = class_info(&class);
        assert_eq!(name, "MUS 499".to_string());
        assert_eq!(size, 0);

        let dev = student_init("Dev".to_string(), "dev".to_string());
        let foo = student_init("Foo".to_string(), "foo".to_string());
        let bar = student_init("Bar".to_string(), "bar".to_string());
        let students = vec![&dev, &foo, &bar];

        let attendance = get_attendance(students.clone(), &class);
        assert_eq!(attendance, None);

        // Insert one student
        class_enrollment(&dev, &mut class);
        let attendance = get_attendance(students.clone(), &class);
        let expected = Some(vec![
            (dev.clone(), true),
            (foo.clone(), false),
            (bar.clone(), false),
        ]);
        assert_eq!(attendance, expected);

        // Insert another student
        class_enrollment(&foo, &mut class);
        let attendance = get_attendance(students.clone(), &class);
        let expected = Some(vec![
            (dev.clone(), true),
            (foo.clone(), true),
            (bar.clone(), false),
        ]);
        assert_eq!(attendance, expected);

        // Insert last student
        class_enrollment(&bar, &mut class);
        let attendance = get_attendance(students, &class);
        let expected = Some(vec![
            (dev.clone(), true),
            (foo.clone(), true),
            (bar.clone(), true),
        ]);
        assert_eq!(attendance, expected);
    }
}
