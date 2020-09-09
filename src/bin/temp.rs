#![no_main]
#![no_std]

use {
    embedded_hal::blocking::delay::DelayMs,
    nrf52840_hal::{
        self as hal,
        gpio::{p0::Parts as P0Parts, Level},
        Rng, Timer,
    },
    nrf_retreat as _, // global logger + panicking-behavior + memory layout
    nrf_smartled::pwm::Pwm,
    smart_leds::{gamma, RGB8},
    smart_leds_trait::SmartLedsWrite,
};

const AVG_SAMPLES: f32 = 25.0;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let board = hal::pac::Peripherals::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let gpios = P0Parts::new(board.P0);
    let mut temp = hal::Temp::new(board.TEMP);

    let sdout = gpios.p0_03.into_push_pull_output(Level::Low);

    let mut leds = Pwm::new(board.PWM0, sdout.degrade());

    let _rng = Rng::new(board.RNG);
    let mut pixels = [RGB8::default(); 16];

    leds.write(pixels.iter().cloned()).ok();

    let mut avg = 0.0;
    loop {
        let temp = temp.measure();
        let temp: f32 = temp.to_num();
        avg = (avg * AVG_SAMPLES + temp) / (AVG_SAMPLES + 1.0);

        defmt::info!("temp = {:f32}°C", avg);
        let mut temp = avg - 20.0;
        if temp < 0.0 {
            temp = 0.0;
        }
        let leds_on = (temp / 10.0) * 16.0;

        for (index, pixel) in pixels.iter_mut().enumerate() {
            let brightness = if leds_on as usize > index {
                0xFF
            } else if (leds_on as usize) < index {
                0
            } else {
                let brightness = leds_on - (leds_on as usize as f32);
                (255.0 * brightness) as u8
            };

            match index {
                0..=3 => pixel.b = brightness,
                4..=6 => pixel.g = brightness,
                7..=9 => {
                    pixel.g = brightness;
                    pixel.r = brightness;
                }
                10..=15 => pixel.r = brightness,
                _ => unreachable!(),
            }
        }

        leds.write(gamma(pixels.iter().cloned())).ok();
        timer.delay_ms(10u32);
    }
}
