use irrigation::read_sensors;
use irrigation::voltage;
//use std::thread;
//use std::time::Duration;


fn main() {
    for (channel, value) in read_sensors().iter().enumerate() {
        let int_value = i16::clone(value);
        let voltage = voltage(int_value);
        println!("Channel {}: value: {} voltage: {}", channel, value, voltage);
        
    }
    // get I2C device back
    
}
