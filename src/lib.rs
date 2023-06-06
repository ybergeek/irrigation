use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use nb::block;
use ads1x1x::{channel, Ads1x1x,DataRate16Bit, FullScaleRange, SlaveAddr};
use ads1x1x::ic::{Ads1115, Resolution16Bit};
use ads1x1x::interface::I2cInterface;
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write};
use std::fs::File;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const CH_1: u8 = 19;
const CH_3: u8 = 20;
const CH_4: u8 = 21;
const CH_2: u8 = 26;


#[derive(Debug, Serialize, Deserialize)]
struct Calibration{
 full_saturation: FullSaturation,
 zero_saturation: ZeroSaturation,
}

#[derive(Debug, Serialize, Deserialize)]
struct FullSaturation{
    channel_zero: i16,
    channel_one: i16,
    channel_two: i16,
    channel_three: i16,
}

#[derive(Debug, Serialize, Deserialize)]
struct ZeroSaturation{
    channel_zero: i16,
    channel_one: i16,
    channel_two: i16,
    channel_three: i16,
}


pub fn write(min: [i16;4],max: [i16;4])->std::io::Result<()>{
    let full_saturation: FullSaturation = FullSaturation{
        channel_zero: min[0],
        channel_one: min[1],
        channel_two: min[2],
        channel_three: min[3],
    };
    
    let zero_saturation: ZeroSaturation = ZeroSaturation{
        channel_zero: max[0],
        channel_one: max[1],
        channel_two: max[2],
        channel_three: max[3],
    };

    let  calibration: Calibration = Calibration{full_saturation,zero_saturation};

    let file: File = File::create("cap_config.json")?;
    let mut writer: BufWriter<File> = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &calibration)?;
    writer.flush()?;
    Ok(())
}


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
pub type Adc = Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

pub fn establish_sensor() -> Ads1x1x<ads1x1x::interface::I2cInterface<I2cdev>, ads1x1x::ic::Ads1115, ads1x1x::ic::Resolution16Bit, ads1x1x::mode::OneShot> {
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
    adc
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

