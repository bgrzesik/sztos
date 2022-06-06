use core::any::Any;

use crate::typed_register;

use super::mmu::MEMORY_MAP_SIZE;

pub struct Granule<const N: usize>;

impl<const N: usize> Granule<N> {
    pub const VALUE: u64 = N as u64;
    pub const SHIFT: u64 = N.trailing_zeros() as u64;

    pub const fn lshift(value: u64) -> u64 {
        value << Self::SHIFT
    }

    pub const fn rshift(value: u64) -> u64 {
        value >> Self::SHIFT
    }
}

pub type Granule512MiB = Granule<{ 512 * 1024 * 1024 }>;
pub type Granule64KiB = Granule<{ 64 * 1024 }>;

const LVL2_TABLES_COUNT: u64 = Granule64KiB::rshift(MEMORY_MAP_SIZE);

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum AccessPermission {
    ReadWriteEL1NoEL0   = 0b00,
    ReadWrite           = 0b01,
    ReadOnlyEL1NoEL0    = 0b10,
    ReadOnly            = 0b11
}

#[repr(u64)]
#[derive(Clone, Copy)]
pub enum Shareability {
    OuterShareable = 0b10,
    InnerShareable = 0b11
}

pub struct DescriptorConfig {
    pub uxn: bool,
    pub pxn: bool,
    pub af: bool,
    pub sh: Shareability,
    pub ap: AccessPermission,
    pub index: u64,
    pub TYPE: bool,
    pub valid: bool
}

typed_register! {
    register DescriptorReg: u64 {
        UXN     @ 54,
        PXN     @ 53,
        ADDR    @ 47 : 16,
        AF      @ 10,
        SH      @ 9 : 8,
        AP      @ 7 : 6,
        INDEX   @ 5 : 2,
        TYPE    @ 1,
        VALID   @ 0
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct TranslationTable<const N: usize> {
    lvl3: [[DescriptorReg; 8192]; N],
    lvl2: [DescriptorReg; N]
}

pub type KernelTranslationTable = TranslationTable<{ LVL2_TABLES_COUNT as usize }>;

trait Address {
    fn physical_address(&self) -> u64;
}

impl<T, const N: usize> Address for [T; N] {
    fn physical_address(&self) -> u64 {
        self as *const T as u64
    }
}

impl<const N: usize> TranslationTable<N> {
    pub fn table_base_address(&self) -> u64 {
        self.lvl2.physical_address()
    }

    pub const fn new() -> Self {
        Self { 
            lvl3: [[DescriptorReg::new(); 8192]; N], 
            lvl2: [DescriptorReg::new(); N]
        }
    }

    pub fn map_one_to_one(&mut self, config: &DescriptorConfig) {
        for (i2, a2) in self.lvl2.iter_mut().enumerate() {
            *a2 = DescriptorReg::from_addr_with_config(
                self.lvl3[i2].physical_address(), 
                config
            );

            for (i3, a3) in self.lvl3[i2].iter_mut().enumerate() {
                *a3 = DescriptorReg::from_addr_with_config(
                    Granule512MiB::lshift(i2 as u64) + Granule64KiB::lshift(i3 as u64),
                    config
                );
            }
        }
    }
}

impl DescriptorReg {
    fn from_addr(addr: u64) -> Self {
        Self { 
            UXN: false, 
            PXN: false, 
            ADDR: Granule64KiB::rshift(addr),
            AF: false, 
            SH: 0, 
            AP: 0, 
            INDEX: 0,
            TYPE: false,
            VALID: false
        }
    }

    fn from_addr_with_config(addr: u64, config: &DescriptorConfig) -> Self {
        Self {
            UXN: config.uxn,
            PXN: config.pxn,
            ADDR: Granule64KiB::rshift(addr),
            AF: config.af,
            SH: config.sh as u64,
            AP: config.ap as u64,
            INDEX: config.index,
            TYPE: config.TYPE,
            VALID: config.valid
        }
    }

    const fn new() -> Self {
        Self { 
            UXN: false, 
            PXN: false, 
            ADDR: 0, 
            AF: false, 
            SH: 0, 
            AP: 0, 
            INDEX: 0,
            TYPE: false, 
            VALID: false 
        }
    }
}
