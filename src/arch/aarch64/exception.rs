
use core::arch::global_asm;
use core::fmt::Write;

global_asm!(include_str!("exception.S"));

#[repr(C)]
struct RegisterDump {
    xn:  [u64; 31],

    sp:  *mut (),
    elr: *mut (),

    esr: u64,
    spsr: u64,
}


#[repr(u64)]
#[allow(unused)]
enum ExceptionType {
    CurrentELSp0Synchronous = 0x00,
    CurrentELSp0Irq         = 0x01,
    CurrentELSp0Fiq         = 0x02,
    CurrentELSp0SError      = 0x03,

    CurrentELSpXSynchronous = 0x10,
    CurrentELSpXIrq         = 0x11,
    CurrentELSpXFiq         = 0x12,
    CurrentELSpXSError      = 0x13,

    LowerELSp0Synchronous   = 0x20,
    LowerELSp0Irq           = 0x21,
    LowerELSp0Fiq           = 0x22,
    LowerELSp0SError        = 0x23,

    LowerELSpXSynchronous   = 0x30,
    LowerELSpXIrq           = 0x31,
    LowerELSpXFiq           = 0x32,
    LowerELSpXSError        = 0x33,
}

use crate::platform::UART0;
use crate::drivers::pl011::*;

#[no_mangle]
unsafe extern "C" fn return_func(a: u64) {
    let mut uart = Uart::new(&UART0, 115200, StopBit::One, Some(Parity::Even));

    uart.reset();

    for _ in 0..a {
        uart.write_str("abcr12123\n");
    }

    loop {}
}

#[no_mangle]
unsafe extern "C" fn arch_exception(regs: &mut RegisterDump, excep: ExceptionType) {
    regs.xn[0] = 10;
    regs.elr = return_func as usize as *mut ();
}
