#![no_std]
#![no_main]
#![feature(exhaustive_patterns)]
#![feature(stmt_expr_attributes)]

//use panic_halt as _; // breakpoint on `rust_begin_unwind` to catch panics
use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f3xx_hal::{pac, prelude::*};


// https://www.we-online.com/catalog/datasheet/157142V12703.pdf


#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = peripherals.RCC.constrain();

    // For determining which bus (ahb) is needed, section 3.2.2 in
    // https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf
    // documents which peripherals are reachable over which buses.
    let gpioe = peripherals.GPIOE.split(&mut reset_and_clock_control.ahb);


    loop {
        asm::nop()
    }
}
