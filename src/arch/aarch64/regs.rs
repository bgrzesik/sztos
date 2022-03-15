use core::{
    stringify,
    concat,
    arch::{global_asm, asm},
};

macro_rules! system_register {
    ($name: ident) => {
        #[inline(always)]
        pub unsafe fn $name() -> u64 {
            let mut reg: u64;
            asm!(
                concat!("mrs {reg}, ", stringify!($name)),
                reg = out(reg) reg
            );
            reg
        }
    };
}

pub struct SystemRegisters;

impl SystemRegisters {
    system_register!(mpidr_el1);
}


pub struct System;

impl System {
    #[inline(always)]
    pub unsafe fn core_id() -> u16 {
        (SystemRegisters::mpidr_el1() & 0x03) as u16
    }
}

