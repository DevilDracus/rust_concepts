use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your Name:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Hello {} welcome to Rust!", input);
        }
        Err(e) => {
            println!("Oops! Something went wrong {}", e);
        }
    }
}
