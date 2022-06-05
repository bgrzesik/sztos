
use core::{
    fmt::Write,
    arch::global_asm,
};
use crate::{
    platform::*,
    syscall::handle_syscall,
};

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

#[repr(u8)]
#[allow(unused)]
enum ExceptionClass {
    Aarch64SVC = 21
}

impl core::convert::TryFrom<u8> for ExceptionClass {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use ExceptionClass::*;

        Ok(match value {
            21 => Aarch64SVC,

            _ => {
                return Err(());
            }
        })
    }
}

#[no_mangle]
unsafe extern "C" fn return_func(a: u64) {
    let mut uart = &mut *UART0.lock();

    uart.reset();

    for _ in 0..a {
        uart.write_str("abcr12123\n");
    }

    loop {}
}

#[no_mangle]
unsafe extern "C" fn arch_exception(regs: &mut RegisterDump, excep: ExceptionType) {
    let ec = (((0b111111 << 26) & regs.esr) >> 26) as u8;
    let ec = ExceptionClass::try_from(ec);
    let iss = 0xffffff & regs.esr;

    match ec {
        Ok(ExceptionClass::Aarch64SVC) => {
            handle_syscall(iss, &mut regs.xn[..8], &mut regs.elr);
        },
        _ => panic!("Unknown ExceptionClass"),
    }

}
