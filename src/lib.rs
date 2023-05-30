use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use nb::block;
use ads1x1x::{channel, Ads1x1x};
use ads1x1x::ic::{Ads1115, Resolution16Bit};
use ads1x1x::interface::I2cInterface;



// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const CH_1: u8 = 19;
const CH_3: u8 = 20;
const CH_4: u8 = 21;
const CH_2: u8 = 26;


pub fn establish_pin1()->  OutputPin{
    Gpio::new().unwrap().get(CH_1).unwrap().into_output()
}

pub fn establish_pin2()-> OutputPin{
    Gpio::new().unwrap().get(CH_2).unwrap().into_output()
}

pub fn establish_pin3()-> OutputPin{
    Gpio::new().unwrap().get(CH_3).unwrap().into_output()
}

pub fn establish_pin4()-> OutputPin{
    Gpio::new().unwrap().get(CH_4).unwrap().into_output()
}
type Adc = Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

pub fn read_sensor0(adc: &mut Adc) -> i16  {
    block!(adc.read(&mut channel::SingleA0)).unwrap()

}

pub fn read_sensor1(adc: &mut Adc) -> i16  {
    block!(adc.read(&mut channel::SingleA1)).unwrap()

}

pub fn read_sensor2(adc: &mut Adc) -> i16 {
    block!(adc.read(&mut channel::SingleA2)).unwrap()

}

pub fn read_sensor3(adc: &mut Adc) -> i16 {
    block!(adc.read(&mut channel::SingleA3)).unwrap()

}

pub fn read_sensor(num: usize, adc: &mut Adc) -> i16 {
    match num{
        0 => return block!(adc.read(&mut channel::SingleA0)).unwrap(),
        1 => return block!(adc.read(&mut channel::SingleA1)).unwrap(),
        2 => return block!(adc.read(&mut channel::SingleA2)).unwrap(),
        3 => return block!(adc.read(&mut channel::SingleA2)).unwrap(),
        _ => return 0,
    }
}

pub fn read_sensors(adc: &mut Adc) -> [i16;4]{
 
   [
        block!(adc.read(&mut channel::SingleA0)).unwrap_or(8091),
        block!(adc.read(&mut channel::SingleA1)).unwrap_or(8091),
        block!(adc.read(&mut channel::SingleA2)).unwrap_or(8091),
        block!(adc.read(&mut channel::SingleA3)).unwrap_or(8091),
    ]
}



pub fn voltage(value: i16)  -> f64{
    (f64::from(value) * 4.096) / f64::from(32767)
}

