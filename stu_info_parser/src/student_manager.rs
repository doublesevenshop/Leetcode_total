use crate::model::Student;

pub struct StudentManager {
    students: Vec<Student>,
}

impl StudentManager {
    pub fn new() -> Self {
        Self { students: Vec::new() }
    }
    pub fn add(&mut self, student: Student) {
        self.students.push(student);
        println!("Student's info has been added");
    }

    pub fn list(&self) {
        for student in &self.students {
            println!("{:?}", student);
        }
    }
}