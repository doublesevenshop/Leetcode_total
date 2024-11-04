// src/model.rs
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Student{
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub gender: String,
    pub grade: u8,
}
pub fn read_from_csv(file_path: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let file = File::open(file_path)?;

    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut students = Vec::new();

    for result in rdr.deserialize() {
        let student: Student = result?;
        students.push(student);
    }
    Ok(students)
}