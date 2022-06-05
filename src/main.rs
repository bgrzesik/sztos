
#![no_std]
#![no_main]
#![feature(const_mut_refs)]


mod arch;
mod panic;
mod drivers;
mod platform;
mod sync;
mod register;
mod syscall;

use core::fmt::Write;

use platform::*;

unsafe fn kernel_start() {
    loop { 
        let s = "ABCDDD";
        core::arch::asm!("
            svc 1
        ",
            in("x0") (s.as_ptr()),
            in("x1") (s.len()));

        loop {}
    }
}
