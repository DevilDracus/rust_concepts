fn main() {
    let mut n: i32 = 0;

    loop{
        n += 1;

        if n % 2 == 0 {
            continue;
        }

        if n > 10 {
            break;
        }
        println!("The value of n is {}", n);
    }
}
