
__boot_addr = 0x80000;
ENTRY(__boot_addr);

EXTERN(__stack_addr);

SECTIONS
{
    . = __boot_addr;
    .text :
    { 
        KEEP(*(.text._start));
        *(.text .text.*)
    }

    .data : { *(.data .data.*) }
    .rodata : { *(.rodata .rodata.*) }

    .bss :
    {
        . = ALIGN(16);
        KEEP(*(.bss .bss.*))
    }



    . = 0x80000000;
    __stack_addr = .;
}
