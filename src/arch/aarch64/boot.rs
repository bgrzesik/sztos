
use core::arch::global_asm;

// aarch64 main entry point
global_asm!(r#"
    .section .text._start

    _start:
        ldr x30, =__stack_addr
        mov sp, x30
        bl arch_start
    loop:
        b loop

    .global _start
    .size _start, . - _start
"#);
