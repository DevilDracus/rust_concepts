use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt")
        .expect("Could not create file!");
    
    file.write_all(b"Write byte buffer to file!")
        .expect("Could not write to file!");

}
