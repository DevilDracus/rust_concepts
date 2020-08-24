//traits are like interfaces in other languages
struct Person{
    name: String,
    age: u8
}

impl ToString for Person{
    fn to_string(&self) -> String{
        return format!("My name is {} and I am {}.", self.name, self.age);
    }
}

fn main() {
    let flo = Person{name: String::from("Florian"),age: 26};

    println!("{}", flo.to_string()); // My name is Florian and I am 26.
}
