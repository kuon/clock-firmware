mod gpio;
mod leds;

pub use crate::leds::Leds;
use chrono::prelude::*;
use std::time::{Duration, SystemTime};
use std::{thread, time};

fn left_digit(n: u8) -> u8 {
    leds::DIGITS[(n / 10) as usize]
}

fn right_digit(n: u8) -> u8 {
    leds::DIGITS[(n % 10) as usize]
}

fn main() {
    let leds = Leds::new();
    leds.enable_output();

    loop {
        let now = Local::now();
        let hour = now.hour() as u8;
        let minute = now.minute() as u8;
        let day = now.day() as u8;
        let month = now.month() as u8;
        let sec = now.second();

        if (55..60).contains(&sec) | (30..35).contains(&sec) {
            leds.set(
                left_digit(day),
                right_digit(day) | leds::DOT,
                left_digit(month),
                right_digit(month),
                0,
            );
        } else {
            leds.set(
                left_digit(hour),
                right_digit(hour),
                left_digit(minute),
                right_digit(minute),
                leds::COLON_BOTTOM | leds::COLON_TOP,
            );
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}
