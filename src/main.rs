
#![no_std]
#![no_main]


mod arch;
mod panic;
mod drivers;
mod platform;
mod register;

use core::fmt::Write;

use platform::UART0;

use drivers::pl011::*;

unsafe fn kernel_start() {
    loop { 
        let mut uart = Uart::new(&UART0, 115200, StopBit::One, Some(Parity::Even));
        uart.reset();
        uart.write_str("abc");

        loop {}
    }
}
