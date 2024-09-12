use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use embedded_alloc::Heap;
use hal::pwm::{Pwm, C1, C2, C3, C4};
use hal::time::MonoTimer;
use hal::{adc::ADC, pac::TIM1};
use rtt_target::{rprintln, rtt_init_print};
use stm32l4xx_hal::gpio::{Analog, Pin, L8};

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn config() -> (
    ADC,
    Delay,
    MonoTimer,
    (Pwm<TIM1, C1>, Pwm<TIM1, C2>, Pwm<TIM1, C3>, Pwm<TIM1, C4>),
    Pin<Analog, L8, 'A', 6>,
    Pin<Analog, L8, 'A', 7>,
) {
    rtt_init_print!();
    rprintln!("Starting...");

    rprintln!("Configuring heap...");
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 8192;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }

    rprintln!("Configuring peripherals...");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    rprintln!("Configuring clock...");
    let clocks = rcc.cfgr.sysclk(64.MHz()).freeze(&mut flash.acr, &mut pwr);

    rprintln!("Configuring timer...");
    let timer = MonoTimer::new(cp.DWT, clocks);

    rprintln!("Configuring ADC...");
    let mut delay = Delay::new(cp.SYST, clocks);
    let adc = ADC::new(
        dp.ADC1,
        dp.ADC_COMMON,
        &mut rcc.ahb2,
        &mut rcc.ccipr,
        &mut delay,
    );

    rprintln!("Configuring GPIOs...");
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let c1 = gpioa
        .pa8
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let c2 = gpioa
        .pa9
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let c3 = gpioa
        .pa10
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let c4 = gpioa
        .pa11
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let a1 = gpioa.pa6.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
    let a2 = gpioa.pa7.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    rprintln!("Configuring PWM...");
    let mut pwm = dp
        .TIM1
        .pwm((c1, c2, c3, c4), 60.Hz(), clocks, &mut rcc.apb2);

    pwm.0.enable();
    pwm.0.set_duty(0);
    pwm.1.enable();
    pwm.1.set_duty(0);
    pwm.2.enable();
    pwm.2.set_duty(0);
    pwm.3.enable();
    pwm.3.set_duty(0);
    rprintln!("Configured!");
    rprintln!("Starting Loop.");

    return (adc, delay, timer, pwm, a1, a2);
}
