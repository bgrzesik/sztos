use crate::kernel_start;

mod boot;

mod regs;
pub use regs::*;

mod instr;
pub use instr::*;


#[no_mangle]
unsafe extern "C" fn arch_start() {
    while System::core_id() != 0 {
        Instr::wfe()
    }

    kernel_start();

    loop {}
}

