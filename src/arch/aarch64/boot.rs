use core::arch::global_asm;

// aarch64 main entry point
global_asm!(
    r#"
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
        mrs x0, HCR_EL2
        orr x0, x0, #(1<<31)
        msr HCR_EL2, x0

        mov x0, #(1 << 9 | 1 << 8 | 1 << 7 | 1 << 6 | 0b0101)
        msr SPSR_EL2, x0

        msr SP_EL1, x30

        adr x0, el1_start
        msr ELR_EL2, x0

        eret


    el1_start:
        ldr x30, =__stack_addr
        mov sp, x30

        bl zero_bss

        adr x0, vector_table
        msr VBAR_EL1, x0

        bl arch_start
    loop:
        wfe
        b loop

    zero_bss:
        ldr x0, =__bss_begin
        ldr x1, =__bss_end

    zero_loop:
        cmp x0, x1
        b.eq zero_done
        stp xzr, xzr, [x0], #16
        b zero_loop

    zero_done:
        ret

    .global _start
    .size _start, . - _start
"#
);
