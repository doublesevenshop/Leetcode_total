// src/model.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Student{
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub gender: String,
    pub grade: u8,
}