use core::{
    stringify,
    concat,
    arch::asm,
};

use crate::typed_register;

typed_register! {
    register DDD: u32 {
        aaaa @ 11
    }
}

macro_rules! system_register {
    ($name: ident, $reg: ident) => {
        #[allow(unused)]
        #[inline(always)]
        pub unsafe fn $name() -> u64 {
            let mut reg: u64;
            asm!(
                concat!("mrs {reg}, ", stringify!($reg)),
                reg = out(reg) reg
            );
            reg
        }
    };
}

pub struct SystemRegisters;

impl SystemRegisters {
    system_register!(mpidr_el1, mpidr_el1);
    system_register!(current_el, CurrentEL);
    system_register!(spsel, SPsel);
}


pub struct System;


#[repr(u8)]
pub enum ExceptionLevel {
    User          = 0x00, // EL0
    Kernel        = 0x01, // EL1
    Hypervisor    = 0x02, // EL2
    SecureMonitor = 0x03, // EL3
    Unknown       = 0xff,
}

impl System {
    #[allow(unused)]
    #[inline(always)]
    pub unsafe fn core_id() -> u16 {
        (SystemRegisters::mpidr_el1() & 0x03) as u16
    }

    #[allow(unused)]
    #[inline(always)]
    pub unsafe fn exception_level() -> ExceptionLevel {
        match SystemRegisters::current_el() {
            0b0000 => ExceptionLevel::User,
            0b0001 => ExceptionLevel::Kernel,
            0b0010 => ExceptionLevel::Hypervisor,
            0b0011 => ExceptionLevel::SecureMonitor,
                 _ => ExceptionLevel::Unknown,
        }
    }
}

