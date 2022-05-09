
#![no_std]
#![no_main]


mod arch;
mod panic;
mod drivers;
mod platform;
mod register;

use platform::UART0;

use drivers::{
    pl011::DR,
    ReadableRegister,
    WritableRegister,
};

unsafe fn kernel_start() {
    loop { 
        let uart = &UART0;

        uart.DR().set_typed(DR {
            DATA: 'a' as u16,
            ..Default::default()
        });

        //loop {}
    }
}
