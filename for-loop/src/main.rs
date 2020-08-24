fn main() {
    let numbers = 1..4;
    
    for i in numbers {
        println!("The number is {}", i);
    }

    let animals = vec!["Rabbit","Dog","Cat"];

    for (index, a) in animals.iter().enumerate() {
        println!("The index is {} and the animal is a {}", index, a);
    }
}
