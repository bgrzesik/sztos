use crate::{arch::*, platform::*, syscall::handle_syscall, typed_register};
use core::{arch::global_asm, fmt::Write};

global_asm!(include_str!("exception.S"));

#[repr(C)]
pub struct RegisterDump {
    pub xn: [u64; 31],

    pub sp: *mut (),
    pub elr: *mut (),

    pub spsr: u64,
}

#[repr(u64)]
#[allow(unused)]
enum ExceptionType {
    CurrentELSp0Synchronous = 0x00,
    CurrentELSp0Irq = 0x01,
    CurrentELSp0Fiq = 0x02,
    CurrentELSp0SError = 0x03,

    CurrentELSpXSynchronous = 0x10,
    CurrentELSpXIrq = 0x11,
    CurrentELSpXFiq = 0x12,
    CurrentELSpXSError = 0x13,

    LowerELSp0Synchronous = 0x20,
    LowerELSp0Irq = 0x21,
    LowerELSp0Fiq = 0x22,
    LowerELSp0SError = 0x23,

    LowerELSpXSynchronous = 0x30,
    LowerELSpXIrq = 0x31,
    LowerELSpXFiq = 0x32,
    LowerELSpXSError = 0x33,
}

#[repr(u8)]
#[allow(unused)]
enum ExceptionClass {
    Aarch64SVC = 0b010101,
    InstractionAbortSameEL = 0b100001,
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
unsafe extern "C" fn arch_exception(regs: &mut RegisterDump, excep: ExceptionType) {
    let esr = System::esr();
    let ec = ExceptionClass::try_from(esr.EC as u8);

    match ec {
        Ok(ExceptionClass::Aarch64SVC) => {
            handle_syscall(esr.ISS, &mut regs.xn[..8], &mut regs.elr);
        }
        Ok(ExceptionClass::InstractionAbortSameEL) => loop {
            Instr::wfe();
        },
        _ => panic!("Unknown ExceptionClass"),
    }
}

extern "C" {
    fn restore_cpu_state(regs: &RegisterDump);

    // There is no return from eret
    fn restore_cpu_state_and_eret(regs: &RegisterDump) -> !;
}

pub unsafe fn switch_to_userspace(elr: *mut (), regs: [u64; 31], sp: *mut ()) {
    assert_eq!(System::exception_level(), ExceptionLevel::Kernel);

    let mut regs = RegisterDump {
        xn: regs,
        sp,
        elr,
        spsr: 0,
    };

    SystemRegisters::set_sp_el0(sp as u64);
    restore_cpu_state_and_eret(&mut regs);
}
