use input::read_input;
use crate::student_manager::StudentManager;
use crate::model::{Student, read_from_csv};

mod model;
mod input;
mod student_manager;


fn main() {
    let file_path = "info.csv";

    match read_from_csv(file_path) {
        Ok(students) => {
            for student in students {
                println!("{:?}", student);
            }
        }
        Err(e) => {
            eprintln!("Error reading CSV:{}", e);
        }
    }
}