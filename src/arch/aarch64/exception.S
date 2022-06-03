



.macro STORE_CPU_STATE

    // Make place for registers
    sub sp, sp, #8 * 33

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

    mrs x1, ELR_EL1
    str x1, [sp, #8 * 30]

    mrs x1, ESR_EL1
    str x1, [sp, #8 * 31]

    mrs x1, SPSR_EL1
    str x1, [sp, #8 * 32]

.endm

.macro EXCEPTION_HANDER excep
    STORE_CPU_STATE
    mov x0, sp
    mov x1, \excep
    bl arch_exception
    eret
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


