use crate::hal::prelude::*;

use hal::pac::TIM1;
use hal::pwm::{Pwm, C1, C2, C3, C4};

pub struct Motors {
    pub pwm: (Pwm<TIM1, C1>, Pwm<TIM1, C2>, Pwm<TIM1, C3>, Pwm<TIM1, C4>),
}

impl Motors {
    pub fn new(pwm: (Pwm<TIM1, C1>, Pwm<TIM1, C2>, Pwm<TIM1, C3>, Pwm<TIM1, C4>)) -> Motors {
        return Motors { pwm };
    }

    pub fn drive(&mut self, mut rate: i32) {
        let throttle = 13000;
        let mut dir = false;
        if rate.is_negative() {
            dir = true;
            rate = -1 * rate;
        }
        if rate == 0 {
            return;
        }
        let u_rate = u16::try_from(rate).ok().unwrap();
        if dir {
            let left_drive = throttle + u_rate;

            self.pwm.0.set_duty(0);
            self.pwm.1.set_duty(left_drive);

            let mut right_drive = 0;
            if u_rate <= throttle {
                right_drive = throttle - u_rate;
            }

            self.pwm.3.set_duty(0);
            self.pwm.2.set_duty(right_drive);
        } else {
            let mut left_drive = 0;
            if u_rate <= throttle {
                left_drive = throttle - u_rate;
            }

            self.pwm.0.set_duty(0);
            self.pwm.1.set_duty(left_drive);

            let right_drive = throttle + u_rate;

            self.pwm.3.set_duty(0);
            self.pwm.2.set_duty(right_drive);
        }
    }
}
