#[allow(unused)]
pub enum XN {
    ExecuteAlways   = 0,
    ExecuteNever    = 1,
}

#[allow(unused)]
pub enum SH {
    OuterShareable  = 0b10,
    InnerShareable  = 0b11,
}

#[allow(unused)]
pub enum AP {
    ReadWriteEL1    = 0b00,
    ReadWrite       = 0b01,
    ReadOnlyEL1     = 0b10,
    ReadOnly        = 0b11,
}
