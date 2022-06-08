use crate::kernel_start;

mod boot;

mod regs;
pub use regs::*;

mod instr;
pub use instr::*;

mod exception;
pub use exception::*;

mod mmu;
pub use mmu::*;

pub static mut KERNEL_TABLE: mmu::TranslationTable4G = mmu::TranslationTable4G::zeroed();

#[no_mangle]
unsafe extern "C" fn arch_start() {
    while System::core_id() != 0 {
        Instr::wfe()
    }

    KERNEL_TABLE.set_to_identity(&PageDescriptor {
        UXN: false,
        PXN: false,

        ADDR: 0,

        AF: true,
        SH: desc::SH::InnerShareable as u64,
        AP: 0b00,
        INDEX: 0b000,
        TYPE: true,
        VALID: true,
    });

    MMU::set_tables(KERNEL_TABLE.base_address(), Some(KERNEL_TABLE.base_address()));
    MMU::enable_mmu();

    kernel_start();

    loop {
        Instr::wfe()
    }
}

pub mod demo {
    pub unsafe fn mmu_swap_pages(page1: u64, page2: u64) {
        super::MMU::swap_pages(&mut super::KERNEL_TABLE, page1, page2);
    }
}
