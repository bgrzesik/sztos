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

macro_rules! system_register_setter {
    ($name: ident, $reg: ident) => {
        #[allow(unused)]
        #[inline(always)]
        pub unsafe fn $name(v: u64) {
            asm!(
                concat!("msr ", stringify!($reg), ", {v}"),
                v = in(reg) v
            );
        }
    };
}

macro_rules! system_register_rw {
    ($name: ident, $set_name: ident, $reg: ident) => {
        system_register!($name, $reg);
        system_register_setter!($set_name, $reg);
    }
}

pub struct SystemRegisters;

impl SystemRegisters {
    system_register!(mpidr_el1, mpidr_el1);
    system_register!(current_el, CurrentEL);

    system_register!(spsel, SPsel);

    system_register_rw!(sp_el0,    set_sp_el0,    SP_EL0);
    system_register_rw!(sp_el1,    set_sp_el1,    SP_EL1);
    system_register_rw!(tcr_el0,   set_tcr_el0,   TCR_EL0);
    system_register_rw!(tcr_el1,   set_tcr_el1,   TCR_EL1);
    system_register_rw!(ttbr0_el0, set_ttbr0_el0, TTBR0_EL0);
    system_register_rw!(ttbr0_el1, set_ttbr0_el1, TTBR0_EL0);
    system_register_rw!(ttbr1_el1, set_ttbr1_el1, TTBR1_EL1);
}


typed_register! {
    register ExceptionSyndrom: u64 {
        ISS2 @ 36:32,
        EC   @ 31:26,
        IL   @ 25,
        ISS  @ 24:0
    }
}

pub struct System;


#[repr(u8)]
#[derive(Eq, PartialEq, Debug)]
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
        match (SystemRegisters::current_el() >> 2) & 0b11 {
            0b0000 => ExceptionLevel::User,
            0b0001 => ExceptionLevel::Kernel,
            0b0010 => ExceptionLevel::Hypervisor,
            0b0011 => ExceptionLevel::SecureMonitor,
                 _ => ExceptionLevel::Unknown,
        }
    }
}

