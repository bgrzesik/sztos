use core::arch::asm;


pub struct Instr;

impl Instr {
    #[inline(always)]
    pub unsafe fn wfe() {
        asm!("wfe")
    }

    #[inline(always)]
    pub unsafe fn eret() {
        asm!("eret")
    }
}
