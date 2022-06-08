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

use crate::platform::*;

pub static mut KERNEL_TABLE: mmu::TranslationTable4G = mmu::TranslationTable4G::zeroed();

#[no_mangle]
unsafe extern "C" fn arch_start() {
    while System::core_id() != 0 {
        Instr::wfe()
    }

    KERNEL_TABLE.init_level2();

    for addr in MMIO_RANGE.step_by(PAGE_SIZE) {
        let desc = PageDescriptor {
            UXN: false,
            PXN: false,

            ADDR: 0,

            AF: true,
            SH: desc::SH::InnerShareable as u64,
            AP: 0b00,
            INDEX: 0b000,
            TYPE: true,
            VALID: true,
        };
        KERNEL_TABLE.maps(addr, addr, Some(&desc));
    }

    for addr in (0x6_0000..0x20_0000).step_by(PAGE_SIZE) {
        let desc = PageDescriptor {
            UXN: false,
            PXN: false,

            ADDR: 0,

            AF: true,
            SH: desc::SH::InnerShareable as u64,
            AP: 0b00,
            INDEX: 0b000,
            TYPE: true,
            VALID: true,
        };
        KERNEL_TABLE.maps(addr, addr, Some(&desc));
    }

    let desc = PageDescriptor {
        UXN: false,
        PXN: false,

        ADDR: 0,

        AF: true,
        SH: desc::SH::InnerShareable as u64,
        AP: 0b01,
        INDEX: 0b000,
        TYPE: true,
        VALID: true,
    };

    for page in &[
        0x2137_0000,
        0x2138_0000,
        0x3000_8000,
        0x3000_8000 - PAGE_SIZE as u64,
    ] {
        KERNEL_TABLE.maps(*page, *page, Some(&desc));
    }

    MMU::set_tables(KERNEL_TABLE.base_address(), None);
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
