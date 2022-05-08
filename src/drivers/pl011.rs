use crate::device_register_map;

device_register_map! {
    device PL011 {
        DR     @ 0x0000 rw u16,
        RSRECR @ 0x0004 rw u32,
        FR     @ 0x0018 rw u32,
        ILPR   @ 0x0020 rw u32,
        IBRD   @ 0x0024 rw u32,
        FBRD   @ 0x0028 rw u32,
        LCRH   @ 0x002c rw u32,
        CR     @ 0x0030 rw u32,
        IFLS   @ 0x0034 rw u32,
        IMSC   @ 0x0038 rw u32,
        RIS    @ 0x003c rw u32,
        MIS    @ 0x0040 rw u32,
        ICR    @ 0x0044 rw u32,
        DMACR  @ 0x0048 rw u32,
        ITCR   @ 0x0080 rw u32,
        ITIP   @ 0x0084 rw u32,
        ITOP   @ 0x0088 rw u32,
        TDR    @ 0x008c rw u32
    }
}

