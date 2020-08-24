// fn main() {
//     let name = String::from("Florian");

//     println!("Character at index 8: {}", match name.chars().nth(8){
//         Some(c) => c.to_string(),
//         None => "No character at index 8!".to_string()
//     });
// }

fn main(){
    println!("Occupation is {}", match get_occupation("Steve") {
        Some(o) => o,
        None => "No occupation found!"
    });
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Florian" => Some("Software Developer"),
        "Bob" => Some("Builder"),
        _ => None
    }
}