
#![no_std]
#![no_main]


mod arch;
mod panic;
mod drivers;
mod platform;
mod register;

use platform::UART0;

use drivers::pl011::Uart;

unsafe fn kernel_start() {
    loop { 
        let mut uart = Uart::new(&UART0, 115200, 0, 1);

        uart.reset();

        for b in b"abc" {
            uart.write_byte(*b);
        }

        loop {}
    }
}
