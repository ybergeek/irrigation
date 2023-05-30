//use std::env;
use irrigation::{read_sensor,  voltage};

use std::thread;
use std::time::Duration;

use linux_embedded_hal::I2cdev;
//use nb::block;
//use ads1x1x::{channel, Ads1x1x, DataRate16Bit, FullScaleRange, SlaveAddr,DynamicOneShot};
use ads1x1x::{Ads1x1x, DataRate16Bit, FullScaleRange, SlaveAddr};

use std::io;
fn main() {
    //let mut max_val: [i16; 4] =[0,0,0,0];
    //println!();println!();let mut min_val: [i16; 4] = [0,0,0,0];

    let dev: I2cdev = I2cdev::new("/dev/i2c-1").unwrap();
    let address: SlaveAddr = SlaveAddr::default();
    let mut adc: Ads1x1x<
        ads1x1x::interface::I2cInterface<I2cdev>,
        ads1x1x::ic::Ads1115,
        ads1x1x::ic::Resolution16Bit,
        ads1x1x::mode::OneShot,
    > = Ads1x1x::new_ads1115(dev, address);
    adc.set_data_rate(DataRate16Bit::Sps860).unwrap();
    adc.set_full_scale_range(FullScaleRange::Within4_096V)
        .unwrap();
    
    let max_val = read_dry( &mut adc);

    let min_val = read_water( &mut adc);
    let _dev =adc.destroy_ads1115();
    
    for i in 0..4{
        println!("min value for channel {i} : {}" ,min_val[i]);
        println!("max value for channel {i} : {}" ,max_val[i]);
        
    }

    
}

fn read_water(  adc: &mut Ads1x1x<ads1x1x::interface::I2cInterface<I2cdev>, ads1x1x::ic::Ads1115, ads1x1x::ic::Resolution16Bit, ads1x1x::mode::OneShot>)-> [i16; 4] {
    let reader = io::stdin();
    let mut buffer: String = String::new();
    let mut index: usize = 0;
    let mut min_val: [i16; 4] = [0,0,0,0];
 
    for _ in 0..4 {
            println!("Is Capacitive Sensor {index} in water? (enter 'y' to proceed): ");
            reader.read_line(&mut buffer).ok().expect("ERRMSG");
            if buffer.trim() == "y" {
              min_val[index] = read_sensor(index, adc);
                println!("------{}\t{}", "raw", "v");
            
                for _ in 1..10 {
                    let tmp_value: i16 = read_sensor(index,  adc);
                    if tmp_value <= min_val[index] {
                        min_val[index] = tmp_value;
                        println!("CHAN {index}: {tmp_value}\t{}", voltage(tmp_value));
                        thread::sleep(Duration::from_millis(500));
                    }
                }
                println!("Min value: {}", min_val[index]);
            } else {
                println!("input was not y {buffer}");
            }
            buffer.clear();
            index += 1 ;
        }
        min_val

}

fn read_dry(  adc: &mut  Ads1x1x<ads1x1x::interface::I2cInterface<I2cdev>, ads1x1x::ic::Ads1115, ads1x1x::ic::Resolution16Bit, ads1x1x::mode::OneShot>) -> [i16; 4]{
    let reader = io::stdin();
    let mut buffer: String = String::new();
    let mut index: usize = 0;
    let mut max_val: [i16; 4] =[0,0,0,0];
    
    for _ in 0..4 {
        println!("Is Capacitive Sensor {index} Dry? (enter 'y' to proceed): ");
        reader.read_line(&mut buffer).ok().expect("ERRMSG");
        if buffer.trim() == "y" {
          max_val[index] = read_sensor(index,adc);
            println!("------{}\t{}", "raw", "v");
        
            for _ in 1..10 {
                let tmp_value: i16 = read_sensor(index, adc);
                if tmp_value >= max_val[index] {
                    max_val[index] = tmp_value;
                    println!("CHAN {index}: {tmp_value}\t{}", voltage(tmp_value));
                    thread::sleep(Duration::from_millis(500));
                }
            }
            println!("Max value: {}", max_val[index]);
        } else {
            println!("input was not y {buffer}");
        }
        buffer.clear();
        index += 1 ;
    }
    max_val
}
