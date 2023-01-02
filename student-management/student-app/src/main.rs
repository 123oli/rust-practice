use student_library::StudentManager;

fn main() {
    let mut manager = StudentManager::new();

    let student1 = student_library::Student {
        name: "Alice".to_owned(),
        age: 18,
        grade: 90,
    };

    let student2 = student_library::Student {
        name: "Bob".to_owned(),
        age: 19,
        grade: 85,
    };

    manager.add_student(1, student1);
    manager.add_student(2, student2);

    let student = manager.get_student(1).unwrap();
    println!("Student 1: {}", student.name);

    let students = manager.get_students();
    println!("Number of students: {}", students.len());
}
