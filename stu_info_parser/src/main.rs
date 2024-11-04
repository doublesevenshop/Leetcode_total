use std::io::{self, Write};

fn main() {
    // 先把空的字符串放到这里
    let mut input = String::new();
    
    loop {
        print!("> ");
        // 确保它能够直接刷新
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}