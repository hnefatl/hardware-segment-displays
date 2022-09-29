use stm32f3xx_hal::{
    gpio::{marker::GpioStatic, Output, PXx, Pin, PushPull, U},
    prelude::_embedded_hal_digital_OutputPin,
};

type SegmentPin = PXx<Output<PushPull>>;

pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}


/// Utility function to convert from an arbitrary compile-time-marked pin to a pin configured for use by `SegmentDisplay`.
pub fn configure_pin<Gpio, const X: u8, Mode>(
    pin: Pin<Gpio, U<X>, Mode>,
    moder: &mut Gpio::MODER,
    otyper: &mut Gpio::OTYPER,
) -> SegmentPin
where
    Gpio: GpioStatic,
    Gpio::Reg: 'static + Sized,
{
    pin.into_push_pull_output(moder, otyper).downgrade().downgrade()
}

/// Macro to further reduce boilerplate when configuring pins.
macro_rules! configure_pin {
    ($gpio:ident, $pin:ident) => {
        crate::segment_display::configure_pin($gpio.$pin, &mut $gpio.moder, &mut $gpio.otyper)
    };
}


/// Wrapper around [this device](https://www.we-online.com/catalog/datasheet/157142V12703.pdf).
pub struct SegmentDisplay {
    /// Segments in order from top-to-bottom, left-to-right, i.e: top, top_left, top_right, middle, bottom_left, ...
    segments: [SegmentPin; Self::PIN_COUNT - 1],
    decimal: SegmentPin,
}
impl SegmentDisplay {
    const PIN_COUNT: usize = 8;

    /// Construct a `SegmentDisplay` from an array of pins, ordered anticlockwise from the bottom left (1-10 in the
    /// spec's circuit diagram), and excluding pins 3 and 8, which are ground pins.
    pub fn new(pins: [SegmentPin; Self::PIN_COUNT]) -> Self {
        let [bottom_left, bottom, bottom_right, decimal, top_right, top, top_left, middle] = pins;
        SegmentDisplay {
            segments: [top, top_left, top_right, middle, bottom_left, bottom_right, bottom],
            decimal,
        }
    }

    pub fn show_digit(&mut self, digit: u8) {
        // Convert each possible digit to an array of on/off values. The order of the array
        // is top-to-bottom, left-to-right (i.e. top, top_left, top_right, middle, bottom_left, ...)
        let pins: [bool; Self::PIN_COUNT - 1] = match digit % 10 {
            0 => [true, true, true, false, true, true, true],
            1 => [false, false, true, false, false, true, false],
            2 => [true, false, true, true, true, false, true],
            3 => [true, false, true, true, false, true, true],
            4 => [false, true, true, true, false, true, false],
            5 => [true, true, false, true, false, true, true],
            6 => [true, true, false, true, true, true, true],
            7 => [true, false,true, false, false, true, false],
            8 => [true, true, true, true, true, true, true],
            9 => [true, true, true, true, false, true, true],
            _ => panic!("invalid number for segment display, programmer error")
        };

        for i in 0..self.segments.len() {
            let Ok(_) = self.segments[i].set_state(pins[i].into());
        }
    }
    pub fn show_digit_enum(&mut self, digit: Digit) {
        self.show_digit(digit as u8)
    }
    pub fn set_decimal(&mut self, on: bool) {
        let Ok(_) = self.decimal.set_state(on.into());
    }
}

pub struct SegmentDisplays<const N: usize> {
    digits: [SegmentDisplay; N],
}
impl<const N: usize> SegmentDisplays<N> {
    /// Construct a `SegmentDisplays` from an array of `SegmentDisplay`s, ordered from least-significant digit to
    /// most-significant.
    pub fn new(digits: [SegmentDisplay; N]) -> Self{
        SegmentDisplays { digits }
    }

    pub fn show_number(&mut self, mut x: u32) {
        for i in 0..N {
            self.digits[i].show_digit((x % 10) as u8);
            x /= 10;
        }
    }
}