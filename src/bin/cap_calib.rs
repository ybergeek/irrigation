//use std::env;
use irrigation::{read_sensor,  voltage,establish_sensor, Adc,write};
use std::thread;
use std::time::Duration;
use std::io;



fn main() ->std::io::Result<()>{
    //let mut max_val: [i16; 4] =[0,0,0,0];
    //println!();println!();let mut min_val: [i16; 4] = [0,0,0,0];

    let mut adc = establish_sensor();
    
    let max_val: [i16; 4] = read_sensors( "max",&mut adc);

    let min_val: [i16; 4] = read_sensors( "min",&mut adc);
    let _dev: linux_embedded_hal::I2cdev =adc.destroy_ads1115();
    
    for i in 0..4{
        println!("min value for channel {i} : {}" ,min_val[i]);
        println!("max value for channel {i} : {}" ,max_val[i]);
        
    }
    write(min_val,max_val)?;
    Ok(())
 
}





fn read_sensors( max_min: &str, adc: &mut Adc)-> [i16; 4] {
    let reader = io::stdin();
    let mut buffer: String = String::new();
    let mut index: usize = 0;
    let mut val: [i16; 4] = [0,0,0,0];
 
    for _ in 0..4 {
            if max_min == "min" { println!("Is Capacitive Sensor {index} in water? (enter 'y' to proceed): ");}
            else  { println!("Is Capacitive Sensor {index} Dry? (enter 'y' to proceed): ");}
        
            
            reader.read_line(&mut buffer).ok().expect("ERRMSG");
            if buffer.trim() == "y" {
                val[index] = read_sensor(index, adc);
                println!("------{}\t{}", "raw", "v");
            
                for _ in 1..10 {
                    let tmp_value: i16 = read_sensor(index,  adc);
                    if tmp_value <= val[index] && max_min == "max" {val[index] = tmp_value;}
                    if tmp_value >= val[index] && max_min == "min" {val[index] = tmp_value;}
                
                    println!("CHAN {index}: {tmp_value}\t{}", voltage(tmp_value));
                    thread::sleep(Duration::from_millis(500));
                
                }
                if max_min == "max" {println!("Max value: {}", val[index]);}
                else {println!("Min value: {}", val[index]);}
            } else {
                println!("input was not y {buffer}");
            }
            buffer.clear();
            index += 1 ;
        }
        val

}

