use std::process::Command;

fn main() {
    //python example.py
    let mut cmd = Command::new("python");
    cmd.arg("example.py");

    // Execute the command
    match cmd.output() {
        Ok(output) => {
            unsafe {
                println!("Output: {}", String::from_utf8_unchecked(output.stdout));
            }            
        },
        Err(e) => {
            println!("There was an Error! {}", e);
        }
    }
}
