fn main() {
    let mut n: i32 = 1;

    while n <= 50 {
        // if n is a multiple of 5
        if n % 5 == 0 {
            println!("The value of n is {}", n);
        }
        n += 1;        
    }
}
