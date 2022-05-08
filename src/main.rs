
#![no_std]
#![no_main]


mod arch;
mod panic;
mod drivers;
mod platform;

use platform::UART0;

use drivers::{
    pl011::DR,
    ReadableRegister,
    WritableRegister,
};

unsafe fn kernel_start() {
    loop { 
        let uart = &UART0;

        let mut dr = DR::default();
        dr.DATA = 'a' as u16;

        uart.DR().set_value(dr.into());

        //loop {}
    }
}
