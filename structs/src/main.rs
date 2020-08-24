// color: red, green, blue
struct Color  {
    red: u8, //u8: 0 - 255
    green: u8,
    blue: u8
}

fn main() {
    let mut bg = Color{red: 255, green: 100, blue: 35};

    bg.blue = 55;
    
    println!("{}, {}, {}", bg.red, bg.green, bg.blue);
}
