
use core::arch::global_asm;

// aarch64 main entry point
global_asm!(r#"
    .section .text._start

    _start:
        mrs x0, MPIDR_EL1
        and x0, x0, #0b11
        cmp x0, xzr
        b.ne loop

        mrs x0, CurrentEL

        cmp x0, #0b1100
        b.eq drop_el3_to_el2

        cmp x0, #0b1000
        b.eq drop_el2_to_el1

        b el1_start


    drop_el3_to_el2:
        mov x0, #(1<<10 | 1) // AArch64 | NonSecure
        msr SCR_EL3, x0

        mov x0, #0b1001      // EL2h
        msr SPSR_EL3, x0

        adr x0, drop_el2_to_el1
        msr ELR_EL3, x0

        eret


    drop_el2_to_el1:
        mov x0, #(1<<31)
        msr hcr_el2, x0

        mov x0, #(1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 0b0101)
        msr SPSR_EL2, x0

        msr SP_EL1, x30

        adr x0, el1_start
        msr ELR_EL2, x0

        eret


    el1_start:
        ldr x30, =__stack_addr
        mov sp, x30

        ldr x0, =vector_table
        msr VBAR_EL1, x0

        bl arch_start
    loop:
        wfe
        b loop

    .global _start
    .size _start, . - _start
"#);
