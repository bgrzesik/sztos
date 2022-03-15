use crate::kernel_start;

mod boot;

mod regs;
use regs::*;

mod instr;
use instr::*;


#[no_mangle]
unsafe extern "C" fn arch_start() {
    while System::core_id() != 0 {
        Instr::wfe()
    }

    kernel_start();

    loop {}
}

