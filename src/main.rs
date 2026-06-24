#![no_std]
#![no_main]

mod hardware;
mod converter;
mod constants;
mod utils;

use cortex_m::{delay::Delay, prelude::_embedded_hal_blocking_delay_DelayUs};
use panic_probe as _;
use defmt_rtt as _;

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use crate::{constants::LEVEL_BUFFER_CAPACITY, converter::{Conductivity, convert}, hardware::Sensor, utils::{decode, get_digit_key, get_key}};

// -----------------------------------------------------------------------------
// Main Application Loop
// -----------------------------------------------------------------------------
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let hw = hardware::init();
    let mut sensor = hw.sensor;

    spawner.spawn(task1(sensor).unwrap());
}

#[embassy_executor::task]
async fn task1(mut sensor: Sensor) {
    let mut level_buf: [bool; LEVEL_BUFFER_CAPACITY] = [false; LEVEL_BUFFER_CAPACITY];
    let mut sample_count: usize = 0;
    cortex_m::asm::delay(10000);
    let _ = sensor.enable_key();
    loop {
        // Read the analog pin (awaits the ADC EOC interrupt)
        let voltage = sensor.key_level_read().await;
        //accumulator += level as u32;
        if let Some(level) = convert(voltage) {
            level_buf[sample_count] = level;
            sample_count += 1;

            // Log once every 1000 samples (~20 milliseconds)
            if sample_count >= LEVEL_BUFFER_CAPACITY {
                //let average = accumulator / sample_count;
                //info!("{:?}", level_buf);

                break;
            }
            cortex_m::asm::delay(10);
        } else {
            sample_count = 0;
        }
        
        // Wait 20 µs
        //Timer::after(Duration::from_micros(20)).await;
    }

    let logic_buf = decode(level_buf);
    if let Some(key) = get_key(logic_buf) {
        let digits = get_digit_key(key);
        info!("{:?}", digits);
    } else {
        info!("Key wasn't found");
    }
}
