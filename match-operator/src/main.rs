// fn main() {
//     let number = 11;

//     match number{
//         1 => println!("It is one!"),
//         2..=20 => println!("There is a whole bunch of them!"),
//         _ => println!("It doesn't match!")
//     }
// }

fn main(){
    let name = "Bob";

    match name{
        "Bob" => println!("Like the Builder!"),
        "Florian" => println!("I've heard this name before!"),
        _ => println!("I don't know your name.")
    }
}
