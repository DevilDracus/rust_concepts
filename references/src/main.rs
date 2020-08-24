fn main() {
    let mut number: u32 = 10;

    //immutable reference
    //let num = &number;

    {
        // mutable reference to number
        let num = &mut number;
        // * is needed to mut through a reference
        *num += 1;
    }

    println!("number is {}", number);
}
