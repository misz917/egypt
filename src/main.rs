use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(l) => println!("Received: {}", l),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
