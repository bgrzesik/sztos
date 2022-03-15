
use core::arch::global_asm;

// aarch64 main entry point
global_asm!(r#"
    .section .text._start

    _start:
        adr x0, __boot_addr
        mov sp, x0
        bl arch_start
    loop:
        b loop

    .global _start
    .size _start, . - _start
"#);

