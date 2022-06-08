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

#[no_mangle]
unsafe extern "C" fn arch_start() {
    while System::core_id() != 0 {
        Instr::wfe()
    }

    MMU::enable();

    kernel_start();

    loop {}
}
