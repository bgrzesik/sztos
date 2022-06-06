
__boot_addr = 0x80000;
ENTRY(__boot_addr);

EXTERN(__stack_addr);

PHDRS
{
    ro PT_LOAD FLAGS(4);
    rx PT_LOAD FLAGS(5);
    rw PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = 0x00000000;

    .boot_stack (NOLOAD) :
    {
        . += __boot_addr;
        __stack_addr = .;
    } : rw

    .text :
    { 
        KEEP(*(.text._start));
        *(.text .text.*)
        . = ALIGN(64K);
        __text_text = .;
    } : rx

    .data : { *(.data .data.*) } : rw
    .rodata : { *(.rodata .rodata.*) } : rw

    .bss (NOLOAD) :
    {
        __bss_begin = .;
        . = ALIGN(16);
        KEEP(*(.bss .bss.*))
        . = ALIGN(64K);
        __bss_end = .;
    } : rw



}
