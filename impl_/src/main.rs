struct Rectangle{
    width: u32,
    height: u32
}

//like extension methods in .NET
impl Rectangle{
    //reference on self
    fn print_description(&self){
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    //function with return value
    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}

fn main() {
    let my_rect = Rectangle{width: 10, height:5};

    // Rectangle: 10 x 5
    my_rect.print_description();

    println!("Rectangle is a square: {}", my_rect.is_square());
}
