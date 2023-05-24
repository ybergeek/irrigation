use irrigation::{read_sensors,voltage,read_sensor0,read_sensor1,read_sensor2, read_sensor3};

use std::thread;
use std::time::Duration;

use linux_embedded_hal::I2cdev;
//use nb::block;
//use ads1x1x::{channel, Ads1x1x, DataRate16Bit, FullScaleRange, SlaveAddr,DynamicOneShot};
use ads1x1x::{ Ads1x1x, DataRate16Bit, FullScaleRange, SlaveAddr};

//use ads1x1x::ic::{Ads1115, Resolution16Bit};
//use ads1x1x::interface::I2cInterface;


fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1115(dev, address);
    adc.set_data_rate(DataRate16Bit::Sps860).unwrap();
    adc.set_full_scale_range(FullScaleRange::Within4_096V).unwrap();

    let mut count = 0u32;
    loop{
        count += 1;
        for (channel, value) in read_sensors(&mut adc).iter().enumerate() {
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
    
    let value0: i16 = read_sensor0(&mut adc );
    let volt0: f64 = voltage(value0);
    println!("Channel 0: value: {} voltage: {}", value0, volt0);
    
    let value1: i16 = read_sensor1(&mut adc );
    let volt1: f64= voltage(value1);
    println!("Channel 1: value: {} voltage: {}", value1, volt1);

    let value2: i16 = read_sensor2(&mut adc );
    let volt2: f64 = voltage(value2);
    println!("Channel 2: value: {} voltage: {}", value2, volt2);
    
    let value3: i16 = read_sensor3(&mut adc );
    let volt3: f64= voltage(value3);
    println!("Channel 3: value: {} voltage: {}", value3, volt3);
    
    let _dev = adc.destroy_ads1115();
}
