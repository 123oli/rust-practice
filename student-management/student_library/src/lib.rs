use std::collections::HashMap;

pub struct Student {
    pub name: String,
    pub age: u8,
    pub grade: u8,
}

pub struct StudentManager {
    pub students: HashMap<u32, Student>
}

impl StudentManager {
    pub fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, id: u32, student: Student) {
        self.students.insert(id, student);
    }

    pub fn remove_student(&mut self, id: u32) -> Option<Student> {
        self.students.remove(&id)
    }

    pub fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    pub fn get_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }
}