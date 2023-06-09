use irrigation::{reader};

fn main() {
    let calibration = reader("../config/cap_config.json").unwrap();
    println!("{:#?}", calibration);
    
}