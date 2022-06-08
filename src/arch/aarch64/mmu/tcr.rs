#[allow(unused)]
pub enum TBI {
    NoTagging   = 0b00,
    TagTTBR0    = 0b01,
    TagTTBR1    = 0b10,
    Tag         = 0b11
}

#[allow(unused)]
pub enum IPS {
    Bits32      = 0b000,
    Bits36      = 0b001,
    Bits40      = 0b010,
    Bits42      = 0b011,
    Bits44      = 0b100,
    Bits48      = 0b101,
    Bits52      = 0b110,
}

#[allow(unused)]
pub enum TG1 {
    Granule16KiB    = 0b01,
    Granule4KiB     = 0b10,
    Granule64KiB    = 0b11,
}

#[allow(unused)]
pub enum TG0 {
    Granule4KiB     = 0b00,
    Granule16KiB    = 0b10,
    Granule64KiB    = 0b01,
}

#[allow(unused)]
pub enum SH {
    NonShareable    = 0b00,
    OuterShareable  = 0b10,
    InnerShareable  = 0b11,
}

#[allow(unused)]
pub enum RGN {
    NonCacheable        = 0b00,
    WbRaWaCacheable     = 0b01,
    WtRaNoWaCacheable   = 0b10,
    WbRaNoWaCacheable   = 0b11,
}

#[allow(unused)]
pub enum EPD {
    TranslationWalk     = 0,
    TranslationFault    = 1,
}

#[allow(unused)]
pub enum A {
    TTBR0Define         = 0,
    TTBR1Define         = 1,
}
