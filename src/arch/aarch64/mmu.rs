use core::arch::asm;

use crate::arch::aarch64::{SystemRegisters, TranslationTableControl};

use super::{KernelTranslationTable, DescriptorConfig, Granule};

pub const MEMORY_MAP_SIZE: u64 = 0xFFFF_FFFF + 1; // 4 GiB
const ADDRESS_SIZE: u64 = 64 - Granule::<{ MEMORY_MAP_SIZE as usize }>::SHIFT;

pub struct MMU;

static _MMU: MMU = MMU;

static mut table0: KernelTranslationTable = KernelTranslationTable::new();
// static mut table1: KernelTranslationTable = KernelTranslationTable::new();

const CONFIG: DescriptorConfig = DescriptorConfig {
    uxn: false,     // user executable
    pxn: true,      // aa.
    sh: super::Shareability::InnerShareable,
    af: true,       // shouldn't matter
    ap: super::AccessPermission::ReadWrite,
    index: 0,
    TYPE: true,     // ???
    valid: true,
};

impl MMU {
    fn tcr_configuration() -> u64 {
        let tcr = TranslationTableControl {
            TBI:    0b00,   // top byte is used in calculation
            IPS:    0b010,  // use 40 bits of virtual address
            
            TG1:    0b11,   // 64 kb granule size
            SH1:    0b11,   // inner shareable
            ORGN1:  0b01,   // write-back rw alloc cacheable
            IRGN1:  0b01,   // aa.
            T1SZ:   ADDRESS_SIZE,
            
            TG0:    0b11,
            SH0:    0b11,
            ORGN0:  0b11,
            IRGN0:  0b11,
            T0SZ:   ADDRESS_SIZE,
        };
        
        tcr.into()
    }

    unsafe fn setup_mair() {
        SystemRegisters::set_mair_el1(
            0x0044_04FFu64
        );
    }

    unsafe fn setup_ttbr0(table_address: u64) {
        SystemRegisters::set_ttbr0_el1(table_address);
    }

    unsafe fn setup_ttbr1(table_address: u64) {
        SystemRegisters::set_ttbr1_el1(table_address);
    }

    unsafe fn setup_tcr() {
        SystemRegisters::set_tcr_el1(
            MMU::tcr_configuration()
        );
    }

    unsafe fn enable_mmu() {
        SystemRegisters::set_sctlr_el1(
            SystemRegisters::sctlr_el1() | 0b1
        );
    }

    unsafe fn setup_registers() {
        MMU::setup_ttbr0(table0.table_base_address());
        // MMU::setup_ttbr1(table1.physical_base_address());
        MMU::setup_mair();
        MMU::setup_tcr();
        asm!("isb"); // force changes to be seen by system

        MMU::enable_mmu();
        asm!("isb");
    }

    pub unsafe fn enable(&self) {
        table0.map_one_to_one(&CONFIG);
        // table1.map_one_to_one(&CONFIG);
        
        Self::setup_registers();
    }
}

pub fn mmu() -> &'static MMU {
    &_MMU
}
