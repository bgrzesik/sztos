
__boot_addr = 0x80000;
__stack_addr = 0x80000;

ENTRY(__boot_addr);

EXTERN(__stack_addr);

SECTIONS
{
    . = __boot_addr;

    .text :
    { 
        KEEP(*(.text._start));
        KEEP(*(.text .text.*)) 
    }

    .data : { KEEP(*(.data .data.*)) }
    .rodata : { KEEP(*(.rodata .rodata.*)) }

    .bss :
    {
        . = ALIGN(16);
        KEEP(*(.bss .bss.*))
    }

    .comment : { KEEP(*(.comment)) }
    .debug : { KEEP(*(.debug*)) }

}
