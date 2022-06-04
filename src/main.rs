
#![no_std]
#![no_main]
#![feature(const_mut_refs)]


mod arch;
mod panic;
mod drivers;
mod platform;
mod sync;
mod register;

use core::fmt::Write;

use platform::*;

unsafe fn kernel_start() {
    loop { 
        core::arch::asm!("svc 1");


        {
            let uart = &mut *UART0.lock();
            uart.reset();

            uart.write_str("abc");
        }


        loop {}
    }
}
