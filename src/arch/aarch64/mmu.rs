use core::arch::asm;

use crate::{
    typed_register,
    arch::*
};

const fn calc_size_shift(size: u64) -> u64 {
    size.trailing_zeros() as u64
}

const SHIFT_4G: u64 = calc_size_shift(4 * 1024 * 1024 * 1024);
const SHIFT_512M: u64 = calc_size_shift(512 * 1024 * 1024);
const SHIFT_64K: u64 = calc_size_shift(64 * 1024);

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
    pub(crate) l3: [[u64; 8192]; N],
    pub(crate) l2: [u64; N]
}

pub type TranslationTable4G = TranslationTable<{ (SHIFT_4G - SHIFT_512M) as usize }>;

impl<const N: usize> TranslationTable<N> {

    pub const fn zeroed() -> Self {
        Self { 
            l3: [[0; 8192]; N],
            l2: [0; N]
        }
    }

    pub fn set_to_identity(&mut self, config: &PageDescriptor) {
        for i2 in 0..self.l2.len() {
            let page = self.l3[i2].as_mut_ptr() as *mut () as u64;
            let page = page >> SHIFT_64K;

            self.l2[i2] = TableDescriptor { ADDR: page, TYPE: true, VALID: true }.into();

            for i3 in 0..self.l3[0].len() {
                let addr = ((i2 << SHIFT_512M) | (i3 << SHIFT_64K)) >> SHIFT_64K;

                self.l3[i2][i3] = PageDescriptor { ADDR: addr as u64, ..*config }.into();
            }
        }
    }
}

static mut IDENTITY_TABLE: TranslationTable4G = TranslationTable4G::zeroed();

pub struct MMU;

impl MMU {
    pub unsafe fn enable() {
        IDENTITY_TABLE.set_to_identity(&PageDescriptor {
            UXN:   false,
            PXN:   false,

            ADDR:  0,

            AF:    true,
            SH:    0b11,
            AP:    0b00,
            INDEX: 0b001,
            TYPE:  true,
            VALID: true,
        });

        SystemRegisters::set_ttbr0_el1(
            IDENTITY_TABLE.l2.as_mut_ptr() as *mut () as u64
        );

        SystemRegisters::set_tcr_el1(TranslationTableControl {
            TBI:   0b00,
            IPS:   0x010,

            TG1:   0b00,
            SH1:   0b00,
            ORGN1: 0b00,
            IRGN1: 0b00,
            EPD1:  true,
            A1:    false,
            T1SZ:  0,

            TG0:   0b01,
            SH0:   0b11,
            ORGN0: 0b01,
            IRGN0: 0b01,
            //
            EPD0:  false,
            T0SZ:  (64 - SHIFT_4G),
        }.into());

        Instr::isb();

        SystemRegisters::set_sctlr_el1(
            SystemRegisters::sctlr_el1() | 0b1
        );

        Instr::isb();
    }
}
