use crate::kernel_start;

mod boot;

mod regs;
pub use regs::*;

mod instr;
pub use instr::*;

mod exception;
pub use exception::*;

mod table;
pub use table::*;

mod mmu;
pub use mmu::*;

#[no_mangle]
unsafe extern "C" fn arch_start() {
    while System::core_id() != 0 {
        Instr::wfe()
    }

    let m = mmu();
    m.enable();

    kernel_start();

    loop {}
}
