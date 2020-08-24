fn main() {
    // outside
    let x: u32 = 10;

    {
        //code block is isolated but has access to data outside
        // y only exists in this scope
        let y: u32 = 5;

        println!("x: {} y: {}", x, y);
    }
}
