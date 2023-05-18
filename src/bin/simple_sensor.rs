use irrigation::read_sensors;
use irrigation::voltage;
use irrigation::read_sensor0;
use std::thread;
use std::time::Duration;


fn main() {
    let mut count = 0u32;
    loop{
        count += 1;
        for (channel, value) in read_sensors().iter().enumerate() {
            let int_value = i16::clone(value);
            let voltage = voltage(int_value);
            println!("Channel {}: value: {} voltage: {}", channel, value, voltage);
            
        }
        // get I2C device back

        thread::sleep(Duration::from_millis(500));
        if count == 2 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
    let value = read_sensor0();
    let voltage = voltage(value);
    println!("Channel 0: value: {} voltage: {}", value, voltage);
      
}
