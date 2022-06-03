
use core::arch::global_asm;

global_asm!(include_str!("exception.S"));

#[repr(C)]
struct RegisterDump {
    xn:  [u64; 30],
    elr: u64,
    esr: u64,
    spsr: u64,
}

#[repr(u64)]
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

#[no_mangle]
unsafe extern "C" fn arch_exception(regs: &mut RegisterDump, excep: ExceptionType) {
    loop {}
}
