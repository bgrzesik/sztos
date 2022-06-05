
.macro STORE_CPU_STATE
    // Make place for registers
    sub sp, sp, #8 * 35

    stp  x0,  x1, [sp, #8 *  0]
    stp  x2,  x3, [sp, #8 *  2]
    stp  x4,  x5, [sp, #8 *  4]
    stp  x6,  x7, [sp, #8 *  6]
    stp  x8,  x9, [sp, #8 *  8]

    stp x10, x11, [sp, #8 * 10]
    stp x12, x13, [sp, #8 * 12]
    stp x14, x15, [sp, #8 * 14]
    stp x16, x17, [sp, #8 * 16]
    stp x18, x19, [sp, #8 * 18]

    stp x20, x21, [sp, #8 * 20]
    stp x22, x23, [sp, #8 * 22]
    stp x24, x25, [sp, #8 * 24]
    stp x26, x27, [sp, #8 * 26]
    stp x28, x29, [sp, #8 * 28]
    str x30,      [sp, #8 * 30]

    mov x0, sp
    str x0, [sp, #8 * 31]

    mrs x0, ELR_EL1
    str x0, [sp, #8 * 32]

    mrs x0, ESR_EL1
    str x0, [sp, #8 * 33]

    mrs x0, SPSR_EL1
    str x0, [sp, #8 * 34]
.endm

.global restore_cpu_state
restore_cpu_state:
    // Make place for registers

    ldr x1,       [x0, #8 * 32]
    msr ELR_EL1, x1

    ldr x1,       [x0, #8 * 34]
    msr SPSR_EL1, x1

    ldr x1,       [x0, #8 * 31]
    mov sp, x1

    // ldr x30,      [x0, #8 * 30]
    ldp x28, x29, [x0, #8 * 28]
    ldp x26, x27, [x0, #8 * 26]
    ldp x24, x25, [x0, #8 * 24]
    ldp x22, x23, [x0, #8 * 22]
    ldp x20, x21, [x0, #8 * 20]

    ldp x18, x19, [x0, #8 * 18]
    ldp x16, x17, [x0, #8 * 16]
    ldp x14, x15, [x0, #8 * 14]
    ldp x12, x13, [x0, #8 * 12]
    ldp x10, x11, [x0, #8 * 10]

    ldp  x8,  x9, [x0, #8 *  8]
    ldp  x6,  x7, [x0, #8 *  6]
    ldp  x4,  x5, [x0, #8 *  4]
    ldp  x2,  x3, [x0, #8 *  2]
    ldp  x0,  x1, [x0, #8 *  0]

    ret

.global restore_cpu_state_and_eret
restore_cpu_state_and_eret:
    bl restore_cpu_state
    eret

.macro EXCEPTION_HANDER excep
    STORE_CPU_STATE
    mov x0, sp
    mov x1, \excep
    bl arch_exception
    mov x0, sp
    b restore_cpu_state_and_eret
.endm

/////////////////////////////////////////////////////////////////////////////

.p2align 9 // 0x100
.global vector_table
vector_table:

.org vector_table + 0x000
curr_el_sp0_sync:
    EXCEPTION_HANDER #0x00

.org vector_table + 0x080
curr_el_sp0_irq:
    EXCEPTION_HANDER #0x01

.org vector_table + 0x100
curr_el_sp0_fiq:
    EXCEPTION_HANDER #0x02

.org vector_table + 0x180
curr_el_sp0_serror:
    EXCEPTION_HANDER #0x03

/////////////////////////////////////////////////////////////////////////////

.org vector_table + 0x200
curr_el_spx_sync:
    EXCEPTION_HANDER #0x10

.org vector_table + 0x280
curr_el_spx_irq:
    EXCEPTION_HANDER #0x11

.org vector_table + 0x300
curr_el_spx_fiq:
    EXCEPTION_HANDER #0x12

.org vector_table + 0x380
curr_el_spx_serror:
    EXCEPTION_HANDER #0x13

/////////////////////////////////////////////////////////////////////////////

.org vector_table + 0x400
low_el_sp0_sync:
    EXCEPTION_HANDER #0x20

.org vector_table + 0x480
low_el_sp0_irq:
    EXCEPTION_HANDER #0x21

.org vector_table + 0x500
low_el_sp0_fiq:
    EXCEPTION_HANDER #0x22

.org vector_table + 0x580
low_el_sp0_serror:
    EXCEPTION_HANDER #0x23

/////////////////////////////////////////////////////////////////////////////

.org vector_table + 0x600
low_el_spx_sync:
    EXCEPTION_HANDER #0x30

.org vector_table + 0x680
low_el_spx_irq:
    EXCEPTION_HANDER #0x31

.org vector_table + 0x700
low_el_spx_fiq:
    EXCEPTION_HANDER #0x32

.org vector_table + 0x780
low_el_spx_serror:
    EXCEPTION_HANDER #0x33

/////////////////////////////////////////////////////////////////////////////


