fn main() {
    // tuple on position 4 is a nested tuple
    let tup1 = (20, "Rust", 3.4, false, (1, 4, 7));

    let tup2 = (45, 6.7, "PC");
    // deconstructing a tuple into different variables
    let (a, b ,c) = tup2;

    // to access a nested tuple the main tuple needs to be in parenthesis
    println!("{}", (tup1.4).1);

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}
