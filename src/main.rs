
#![no_std]
#![no_main]

mod arch;
mod panic;
mod drivers;
mod platform;

use platform::UART0;

unsafe fn kernel_start() {
    loop { }
}
