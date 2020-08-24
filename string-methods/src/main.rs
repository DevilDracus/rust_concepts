fn main() {
    // Replace
    {
        let my_string = String::from("Rust is fantastic!");
        println!("After replace: {}", my_string.replace("fantastic","great"));
    }

    // Lines
    {
        let my_string = String::from("The weather is\nnice\noutside!");

        for line in my_string.lines(){
            println!("[ {} ]",line);
        }
    }

    // Split
    {
        let my_string = String::from("Hello+world+I+am+learing+Rust+!");

        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        println!("At index 5: {}", tokens[5]);
    }

    // Trim
    {
        let my_string = String::from("       My name is Florian      \n\r");
        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
    }

    // Chars
    {
        let my_string = String::from("Hello World, we are still learing Rust!");

        // Get character at index
        println!("{}",my_string);
        match my_string.chars().nth(4){
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4!")
        }
    }
}
