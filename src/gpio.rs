use libc::{c_int, c_void, off_t, size_t};
use std::ffi::CString;

const GPIO_BASE_REGISTERS: [off_t; 3] = [0x44E0_7000, 0x4804_C000, 0x481A_C000];
const GPIO_REGISTER_SIZE: size_t = 0xFFF;

const GPIO_OE_REGISTER: isize = 0x134;
const GPIO_DATAOUT_REGISTER: isize = 0x13C;
const GPIO_DATAIN_REGISTER: isize = 0x138;

#[derive(Debug)]
pub enum PinValue {
    Low = 0,
    High = 1,
}

#[derive(Debug)]
pub enum PinMode {
    Output,
    Input,
}

#[derive(Debug)]
pub enum BitOrder {
    LSBFirst,
    MSBFirst,
}

pub struct Gpio {
    fd: c_int,
    base: *mut c_void,
    oe: *mut u32,
    dataout: *mut u32,
    datain: *mut u32,
}

pub struct Pin<'a> {
    gpio: &'a Gpio,
    index: usize,
    invert: bool,
}

impl Drop for Gpio {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.base, GPIO_REGISTER_SIZE);
            libc::close(self.fd);
        };
    }
}

impl Gpio {
    pub fn open(index: usize) -> Gpio {
        let path = CString::new("/dev/mem").unwrap();
        let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDWR) };

        if fd < 0 {
            panic!("Cannot open memory device");
        }

        let base = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                GPIO_REGISTER_SIZE,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                GPIO_BASE_REGISTERS[index],
            )
        };

        if base.is_null() {
            panic!("Cannot map GPIO");
        }

        let oe: *mut u32 = unsafe { base.offset(GPIO_OE_REGISTER) as *mut u32 };
        let dataout: *mut u32 =
            unsafe { base.offset(GPIO_DATAOUT_REGISTER) as *mut u32 };
        let datain: *mut u32 =
            unsafe { base.offset(GPIO_DATAIN_REGISTER) as *mut u32 };

        Gpio {
            fd,
            base,
            oe,
            dataout,
            datain,
        }
    }

    pub fn pin_mode(&self, bit_index: usize, mode: PinMode) {
        let mut bits = unsafe { std::ptr::read_volatile(self.oe) };
        bits = match mode {
            PinMode::Input => bits & !(1 << bit_index),
            PinMode::Output => bits | (1 << bit_index),
        };
        bits &= !(1 << bit_index);
        unsafe { std::ptr::write_volatile(self.oe, bits) };
    }

    pub fn digital_write(&self, bit_index: usize, value: PinValue) {
        let mut bits = unsafe { std::ptr::read_volatile(self.dataout) };
        bits = match value {
            PinValue::Low => bits & !(1 << bit_index),
            PinValue::High => bits | (1 << bit_index),
        };
        unsafe { std::ptr::write_volatile(self.dataout, bits) };

    }

    pub fn digital_read(&self, bit_index: usize) -> PinValue {
        let bits = unsafe { std::ptr::read_volatile(self.datain) };
        if bits & (1 << bit_index) != 0 {
            PinValue::High
        } else {
            PinValue::Low
        }
    }
}

impl Pin<'_> {
    pub fn new(gpio: &Gpio, index: usize, invert: bool) -> Pin {
        Pin { gpio, index, invert }
    }
    pub fn digital_read(&self) -> PinValue {
        let value = self.gpio.digital_read(self.index);
        if self.invert {
            match value {
                PinValue::Low => PinValue::High,
                PinValue::High => PinValue::Low,
            }
        } else {
            value
        }
    }
    pub fn digital_write(&self, value: PinValue) {
        let value = if self.invert {
            match value {
                PinValue::Low => PinValue::High,
                PinValue::High => PinValue::Low,
            }
        } else {
            value
        };

        self.gpio.digital_write(self.index, value);
    }
    pub fn mode(&self, mode: PinMode) {
        self.gpio.pin_mode(self.index, mode);
    }

    pub fn shift_out(
        data_pin: &Pin,
        clock_pin: &Pin,
        bit_order: BitOrder,
        value: u8,
    ) {
        for i in 0..8 {
            let value = match bit_order {
                BitOrder::LSBFirst => {
                    if (value & (1 << i)) != 0 {
                        PinValue::High
                    } else {
                        PinValue::Low
                    }
                }
                BitOrder::MSBFirst => {
                    if (value & (1 << (7 - i))) != 0 {
                        PinValue::High
                    } else {
                        PinValue::Low
                    }
                }
            };
            data_pin.digital_write(value);

            clock_pin.digital_write(PinValue::High);
            clock_pin.digital_write(PinValue::Low);
        }
    }
}
