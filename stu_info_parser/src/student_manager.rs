use crate::model::Student;
use std::collections::HashMap;
use std::fs::File;
use std::collections::HashSet;
use serde_json::to_writer_pretty;

pub struct StudentManager {
    students: HashMap<u32, Student>, // 使用 HashMap 提升查询效率
}

impl StudentManager {
    pub fn new(students: Vec<Student>) -> Self {
        let map = students.into_iter().map(|s| (s.id, s)).collect();
        Self { students: map }
    }

    pub fn count(&self) -> usize {
        self.students.len()
    }

    pub fn find_by_ids(&self, ids: &[u32]) -> Vec<Option<&Student>> {
        ids.iter().map(|id| self.students.get(id)).collect()
    }

    pub fn count_by_gender(&self) -> (usize, usize) {
        let male = self.students.values().filter(|s| s.gender == "M").count();
        let female = self.students.values().filter(|s| s.gender == "F").count();
        (male, female)
    }

    pub fn export_to_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        to_writer_pretty(file, &self.students)?;
        Ok(())
    }
    pub fn find_by_names(&self, names: &[&str]) -> Vec<Option<&Student>> {
        names
            .iter()
            .map(|&name| {
                self.students
                    .values()
                    .find(|student| student.name == name)
                    .map(|student| student)
            }).collect()
    }
}