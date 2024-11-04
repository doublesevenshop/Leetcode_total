use crate::student_manager::StudentManager;
use crate::commands::*;

mod model;
mod input;
mod commands;
mod student_manager;


fn main() {
    let mut manager = StudentManager::new(Vec::new());

    // 首先实现一个解析
    loop {
        let command = input::read_input();
        if command.trim() == "exit" {
            println!("感谢您的使用！");
            break;
        }

        match execute_command(command, &mut manager) {
            Ok(message) => println!("{}", message),
            Err(e) => eprintln!("Error:{}", e),
        }
    }

}