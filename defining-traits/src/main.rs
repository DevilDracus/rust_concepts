struct Person{
    name: String,
    age: u8
}

trait HasVoiceBox {
    // Speak
    fn speak(&self);
    // Check if can speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person{
    fn speak(&self){
        println!("Hello, my name is {}.", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        else{
            return false;
        }
    }
}

fn main() {
    let person = Person { name: String::from("Steve"), age: 35};

    if person.can_speak(){
        person.speak();
    }
    else{
        println!("{} is to young to speak!", person.name);
    }
}
