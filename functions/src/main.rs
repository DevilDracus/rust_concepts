fn main() {
    print_numbers_to(10);
}

fn print_numbers_to(number:u32){
    for i in 1..number{
        if is_even(i){
            println!("The number is {} is even!", i);
        }
        else{
            println!("The number is {} is odd!", i);
        }
    }
}


fn is_even(number:u32) -> bool {
    return number % 2 == 0;
}