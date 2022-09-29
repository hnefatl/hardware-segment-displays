#![no_std]
#![no_main]
#![feature(exhaustive_patterns)]
#![feature(stmt_expr_attributes)]

//use panic_halt as _; // breakpoint on `rust_begin_unwind` to catch panics
use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f3xx_hal::{pac, prelude::*};

#[macro_use]
mod segment_display;
use segment_display::{SegmentDisplay, SegmentDisplays};

fn sleep(seconds: f32) {
    asm::delay((seconds * 8_000_000f32) as u32)
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = peripherals.RCC.constrain();

    // For determining which bus (ahb) is needed, section 3.2.2 in
    // https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf
    // documents which peripherals are reachable over which buses.
    let mut gpiob = peripherals.GPIOB.split(&mut reset_and_clock_control.ahb);
    let mut gpioc = peripherals.GPIOC.split(&mut reset_and_clock_control.ahb);
    let mut gpiod = peripherals.GPIOD.split(&mut reset_and_clock_control.ahb);
    let mut gpioe = peripherals.GPIOE.split(&mut reset_and_clock_control.ahb);

    let mut display = SegmentDisplays::new([
        SegmentDisplay::new([
            configure_pin!(gpiob, pb14),
            configure_pin!(gpiob, pb12),
            // pb10 is ground
            configure_pin!(gpioe, pe14),
            configure_pin!(gpioe, pe12),
            configure_pin!(gpioe, pe13),
            configure_pin!(gpioe, pe15),
            // pb11 is ground
            configure_pin!(gpiob, pb13),
            configure_pin!(gpiob, pb15),
        ]),
        SegmentDisplay::new([
            configure_pin!(gpioc, pc7),
            configure_pin!(gpiod, pd14),
            // pd12 is ground
            configure_pin!(gpiod, pd10),
            configure_pin!(gpiod, pd8),
            configure_pin!(gpiod, pd9),
            configure_pin!(gpiod, pd11),
            // pd13 is ground
            configure_pin!(gpiod, pd15),
            configure_pin!(gpioc, pc6),
        ]),
    ]);

    let mut counter = 0;
    loop {
        display.show_number(counter);
        counter += 1;

        sleep(0.1);
    }
}
