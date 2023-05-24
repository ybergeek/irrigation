//use std::env;

use std::io;
fn main() {
    let reader = io::stdin();
    let mut buffer: String = String::new();

    println!("Is Capacitive Sensor Dry? (enter 'y' to proceed): ");
    reader.read_line(&mut buffer)
        .ok()
        .expect("ERRMSG");
    if buffer == "y"{
        println!("input was y {buffer}");
    }else {
        println!("input was not y
         {buffer}");
    }
}


    
