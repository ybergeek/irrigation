use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use nb::block;
use ads1x1x::{channel, Ads1x1x, DataRate16Bit, FullScaleRange, SlaveAddr};
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

pub fn read_sensors() -> [i16;4]{
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1115(dev, address);
    adc.set_data_rate(DataRate16Bit::Sps860).unwrap();
    adc.set_full_scale_range(FullScaleRange::Within4_096V)
        .unwrap();

    let values =[
        block!(adc.read(&mut channel::SingleA0)).unwrap_or(8091),
        block!(adc.read(&mut channel::SingleA1)).unwrap_or(8091),
        block!(adc.read(&mut channel::SingleA2)).unwrap_or(8091),
        block!(adc.read(&mut channel::SingleA3)).unwrap_or(8091),
    ];
    let dev_ = adc.destroy_ads1115();
    values
}



pub fn voltage(value: i16)  -> f64{
    (f64::from(value) * 4.096) / f64::from(32767)
}

