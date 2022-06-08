use core::mem;

use crate::{
    typed_register,
    arch::*
};

const fn calc_size_shift(size: u64) -> u64 {
    size.trailing_zeros() as u64
}

mod desc;

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

// according to: https://armv8-ref.codingbelief.com/en/chapter_d4/d44_1_memory_access_control.html
// and: https://armv8-ref.codingbelief.com/en/chapter_d4/d43_3_memory_attribute_fields_in_the_vmsav8-64_translation_table_formats_descriptors.html
typed_register! {
    register TableDescriptor: u64 {
        AP      @ 62:61,
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
            
            self.l2[i2] = TableDescriptor { 
                AP: desc::AP::ReadWrite as u64,
                ADDR: page, 
                TYPE: true, 
                VALID: true 
            }.into();

            for i3 in 0..self.l3[0].len() {
                let addr = ((i2 << SHIFT_512M) | (i3 << SHIFT_64K)) >> SHIFT_64K;

                self.l3[i2][i3] = PageDescriptor { ADDR: addr as u64, ..*config }.into();
            }
        }
    }

    pub fn table_index_from_address(&self, address: usize) -> (usize, usize) {
        (
            (address >> SHIFT_512M) & ((1 << N) - 1),
            (address >> SHIFT_64K) & (self.l3.len() - 1)
        )
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
            SH:    desc::SH::InnerShareable as u64,
            AP:    desc::AP::ReadWrite as u64,
            INDEX: 0b001,
            TYPE:  true,
            VALID: true,
        });

        SystemRegisters::set_ttbr0_el1(
            IDENTITY_TABLE.l2.as_mut_ptr() as *mut () as u64
        );

        SystemRegisters::set_tcr_el1(TranslationTableControl {
            TBI:   desc::TBI::NoTagging as u64,
            IPS:   desc::IPS::Bits40 as u64,

            TG1:   desc::TG1::Granule64KiB as u64,
            SH1:   desc::SH::InnerShareable as u64,
            ORGN1: desc::RGN::NonCacheable as u64,
            IRGN1: desc::RGN::NonCacheable as u64,
            EPD1:  desc::EPD::TranslationWalk as u64 != 0,
            A1:    desc::A::TTBR0Define as u64 != 0,
            T1SZ:  0,

            TG0:   desc::TG0::Granule64KiB as u64,
            SH0:   desc::SH::InnerShareable as u64,
            ORGN0: desc::RGN::NonCacheable as u64,
            IRGN0: desc::RGN::NonCacheable as u64,
            //
            EPD0:  desc::EPD::TranslationWalk as u64 != 0,
            T0SZ:  (64 - SHIFT_4G),
        }.into());

        Instr::isb();

        SystemRegisters::set_sctlr_el1(
            SystemRegisters::sctlr_el1() | 0b1
        );

        Instr::isb();
    }

    pub unsafe fn swap_pages(page1: u64, page2: u64) {    
        let (p1l2, p1l3) = IDENTITY_TABLE.table_index_from_address(page1 as usize);
        let (p2l2, p2l3) = IDENTITY_TABLE.table_index_from_address(page2 as usize);
        
        // Make sure our write is in
        Instr::dsb();
        // Invalidate TLB Entries for given adressess
        // for some reason, ALLE1 does not work (execution is trapped by panic handler)
        // core::arch::asm!("TLBI  ALLE1");
        core::arch::asm!("tlbi  VAE1, x0", in("x0") (page1));
        core::arch::asm!("tlbi  VAE1, x1", in("x1") (page2));
        // core::arch::asm!("dsb   ISH");
        Instr::dsb();
        Instr::isb();

        mem::swap(&mut IDENTITY_TABLE.l3[p1l2][p1l3], &mut IDENTITY_TABLE.l3[p2l2][p2l3]);
    }
}
