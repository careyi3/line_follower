use crate::hal::prelude::*;
use alloc::vec::Vec;
use hal::adc::ADC;
use stm32l4xx_hal::gpio::{Analog, Pin, L8};

pub struct Sensor {
    pub adc: ADC,
    pub left: Pin<Analog, L8, 'A', 6>,
    pub right: Pin<Analog, L8, 'A', 7>,
    offset: i32,
}

impl Sensor {
    pub fn new(adc: ADC, left: Pin<Analog, L8, 'A', 6>, right: Pin<Analog, L8, 'A', 7>) -> Sensor {
        return Sensor {
            adc,
            left,
            right,
            offset: 0,
        };
    }

    pub fn read(&mut self) -> i32 {
        let l = i32::from(self.adc.read(&mut self.left).unwrap());
        let r = i32::from(self.adc.read(&mut self.right).unwrap());
        return l - r - self.offset;
    }

    pub fn calibrate(&mut self) {
        let mut ls = Vec::new();
        let mut rs = Vec::new();
        let n = 1000;
        for _ in 0..n - 1 {
            let l = self.adc.read(&mut self.left).unwrap();
            let r = self.adc.read(&mut self.right).unwrap();
            ls.push(l);
            rs.push(r);
        }

        let mut sum_l: i32 = 0;
        let mut sum_r: i32 = 0;
        for i in 0..n - 1 {
            sum_l += i32::from(ls[i]);
            sum_r += i32::from(rs[i]);
        }
        self.offset = (sum_l / n as i32) - (sum_r / n as i32)
    }
}
