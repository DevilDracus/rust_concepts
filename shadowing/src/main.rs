fn main() {
    let mut x: u32 = 10;

    {
        // only assigned in this code block through the use of let (new assigned)
        let x: u32 = 15;
    }

    println!("x is {}", x);
}
