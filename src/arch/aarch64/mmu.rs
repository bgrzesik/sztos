use core::arch::asm;

use crate::{
    typed_register,
    arch::*
};

pub struct Granule<const N: usize>;

impl<const N: usize> Granule<N> {
    pub const SHIFT: u64 = N.trailing_zeros() as u64;
}

pub type Granule512MiB = Granule<{ 512 * 1024 * 1024 }>;
pub type Granule64KiB = Granule<{ 64 * 1024 }>;

typed_register! {
    register PageDescriptor: u64 {
        UXN     @ 54,
        PXN     @ 53,
        ADDR    @ 47:16,
        AF      @ 10,
        SH      @ 9:8,
        AP      @ 7:6,
        INDEX   @ 5:2,
        TYPE    @ 1,
        VALID   @ 0
    }
}
 
typed_register! {
    register TableDescriptor: u64 {
        ADDR    @ 47:16,
        TYPE    @ 1,
        VALID   @ 0
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct TranslationTable<const N: usize> {
    pub(crate) lvl3: [[u64; 8192]; N],
    pub(crate) lvl2: [u64; N]
}

pub type KernelTranslationTable = TranslationTable<{ 0x1_0000_0000 >> Granule512MiB::SHIFT }>;

impl<const N: usize> TranslationTable<N> {

    pub const fn new() -> Self {
        Self { 
            lvl3: [[0; 8192]; N], 
            lvl2: [0; N]
        }
    }

    pub fn map_one_to_one(&mut self, config: &PageDescriptor) {
        for i2 in 0..self.lvl2.len() {
            let page = self.lvl3[i2].as_mut_ptr() as *mut () as u64;
            let page = page >> Granule64KiB::SHIFT;

            self.lvl2[i2] = TableDescriptor { ADDR: page, TYPE: true, VALID: true }.into();

            for i3 in 0..self.lvl3[0].len() {
                // let addr = ((i2 << Granule512MiB::SHIFT) + (i3 << Granule64KiB::SHIFT)) as u64;
                // let addr = addr >> Granule64KiB::SHIFT;
                let addr = (i2 * (512 * 1024 * 1024) + i3 * (64 * 1024)) as u64;
                let addr = addr >> Granule64KiB::SHIFT;

                self.lvl3[i2][i3] = PageDescriptor { ADDR: addr, ..*config }.into();
            }
        }
    }
}

static mut table0: KernelTranslationTable = KernelTranslationTable::new();
// static mut table1: KernelTranslationTable = KernelTranslationTable::new();


pub struct MMU;

impl MMU {
    pub unsafe fn enable() {
        asm!("dsb ishst");
        asm!("dsb ish");
        asm!("isb");

        table0.map_one_to_one(&PageDescriptor {
            UXN: false,
            PXN: false,

            ADDR: 0,

            AF: true,
            SH: 0b11,
            AP: 0b00,
            INDEX: 0b001,
            TYPE: true,
            VALID: true,
        });

        SystemRegisters::set_ttbr0_el1(
            table0.lvl2.as_mut_ptr() as *mut () as u64
        );

        SystemRegisters::set_tcr_el1(
            0x0000000200807520
        );

        // SystemRegisters::set_tcr_el1(TranslationTableControl {
        //     TBI:    0b00,
        //     IPS:    0b010,

        //     TG1:    0b11,
        //     SH1:    0b11,
        //     ORGN1:  0b01,
        //     IRGN1:  0b01,
        //     EPD1:   true,
        //     A1:     false,
        //     T1SZ:   64 - Granule64KiB::SHIFT,

        //     TG0:    0b01,
        //     SH0:    0b11,
        //     ORGN0:  0b01,
        //     IRGN0:  0b01,
        //     EPD0:   false,
        //     T0SZ:   64 - Granule64KiB::SHIFT,
        // }.into());

        // Instr::isb();

        // SystemRegisters::set_mair_el1(
        //     // 0x0044_04FFu64
        //     0x000000000000ff04
        // );

        Instr::isb();

        SystemRegisters::set_sctlr_el1(
            SystemRegisters::sctlr_el1() | 0b1
        );

        Instr::isb();
    }
}
