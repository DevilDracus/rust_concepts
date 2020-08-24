fn main() {
    let mut my_string = String::from("Hello, world!");

    // Length
    println!("Length: {}", my_string.len());

    // Is Empty? 
    println!("String is Empty? {}", my_string.is_empty());

    // Split on whitespace
    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    // Contains
    println!("Does the string contain 'Hello'? {}", my_string.contains("Hello"));

    my_string.push_str(" Welcome to Rust strings!");
    println!("{}", my_string);
}
