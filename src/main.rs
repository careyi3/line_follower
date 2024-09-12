#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;

use panic_rtt_target as _;
extern crate stm32l4xx_hal as hal;

use crate::hal::prelude::*;
use crate::rt::entry;
use rtt_target::rprintln;

extern crate alloc;

mod config;
mod motors;
mod pid;
mod sensor;

#[entry]
fn main() -> ! {
    let (adc, mut delay, _timer, pwm, left, right) = config::config();

    let mut sensor = sensor::Sensor::new(adc, left, right);
    sensor.calibrate();

    let mut motors = motors::Motors::new(pwm);

    let kp = 310.00;
    let ki = 0.05;
    let kd = 0.01;

    let mut pid = pid::PID::new(kp, ki, kd);

    loop {
        let value = sensor.read();
        rprintln!("Sensor: {}", value);
        let error = 0.0 - value as f32;

        //let drive = pid.compute(error, 0.000016);
        let drive = pid.compute(error, 0.001);
        rprintln!("Drive: {}", drive);
        motors.drive(drive as i32);

        delay.delay_ms(1_u32);
    }
}
