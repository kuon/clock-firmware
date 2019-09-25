pub use crate::gpio::PinMode::{Input, Output};
pub use crate::gpio::PinValue::{High, Low};
pub use crate::gpio::{BitOrder, Gpio, Pin, PinMode, PinValue};

pub struct Leds {
    gpio0: Gpio,
    gpio1: Gpio,
    gpio2: Gpio,
}

// 7 segment led bit map order
//       __2
//      4  0
//     /_6/
//    1  5
//   /_3/ .7
// colon bit map order, bottom 0, top 2

pub const TOP_RIGHT: u8 = 1 << 0;
pub const BOTTOM_LEFT: u8 = 1 << 1;
pub const TOP: u8 = 1 << 2;
pub const BOTTOM: u8 = 1 << 3;
pub const TOP_LEFT: u8 = 1 << 4;
pub const BOTTOM_RIGHT: u8 = 1 << 5;
pub const MIDDLE: u8 = 1 << 6;
pub const COLON_TOP: u8 = 1 << 2;
pub const COLON_BOTTOM: u8 = 1 << 0;

pub const DOT: u8 = 1 << 7;

pub const DIGITS: [u8; 10] = [
    BOTTOM_LEFT | TOP_LEFT | TOP | TOP_RIGHT | BOTTOM_RIGHT | BOTTOM, // 0
    TOP_RIGHT | BOTTOM_RIGHT,                                         // 1
    TOP | TOP_RIGHT | MIDDLE | BOTTOM_LEFT | BOTTOM,                  // 2
    TOP | TOP_RIGHT | BOTTOM_RIGHT | BOTTOM | MIDDLE,                 // 3
    TOP_LEFT | MIDDLE | TOP_RIGHT | BOTTOM_RIGHT,                     // 4
    TOP | TOP_LEFT | MIDDLE | BOTTOM_RIGHT | BOTTOM,                  // 5
    TOP | TOP_LEFT | MIDDLE | BOTTOM_RIGHT | BOTTOM | BOTTOM_LEFT,    // 6
    TOP | TOP_RIGHT | BOTTOM_RIGHT,                                   // 7
    TOP | TOP_LEFT | MIDDLE | TOP_RIGHT | BOTTOM_LEFT | BOTTOM | BOTTOM_RIGHT, // 8
    TOP | TOP_LEFT | MIDDLE | TOP_RIGHT | BOTTOM | BOTTOM_RIGHT, // 9
];

impl Leds {
    pub fn new() -> Leds {
        let gpio0 = Gpio::open(0);
        let gpio1 = Gpio::open(1);
        let gpio2 = Gpio::open(2);

        let leds = Leds {
            gpio0,
            gpio1,
            gpio2,
        };

        leds.sdi().mode(Output);
        leds.clk().mode(Output);
        leds.oe().mode(Output);
        leds.le().mode(Output);

        leds.oe().digital_write(High);

        leds
    }
    fn sdi(&self) -> Pin {
        Pin::new(&self.gpio1, 12, true)
    }
    fn oe(&self) -> Pin {
        Pin::new(&self.gpio1, 14, true)
    }
    fn clk(&self) -> Pin {
        Pin::new(&self.gpio0, 26, true)
    }
    fn le(&self) -> Pin {
        Pin::new(&self.gpio2, 1, true)
    }

    pub fn enable_output(&self) {
        self.oe().digital_write(Low);
    }
    pub fn disable_output(&self) {
        self.oe().digital_write(High);
    }
    pub fn set(&self, a: u8, b: u8, c: u8, d: u8, e: u8) {
        let data: [u8; 5] = [a, b, c, d, e];

        // Set LED Value
        self.le().digital_write(Low);
        self.sdi().digital_write(Low);
        self.clk().digital_write(Low);

        self.sdi().digital_write(High);

        for byte in &data {
            Pin::shift_out(&self.sdi(), &self.clk(), BitOrder::LSBFirst, *byte);
        }

        //sdi.digital_write(Low);
        self.clk().digital_write(Low);

        self.le().digital_write(High);
        self.le().digital_write(Low);
    }
}
