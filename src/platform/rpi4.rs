use crate::drivers::pl011;

#[allow(dead_code)]
pub static UART0: pl011::Registers = pl011::Registers(0x7e201000);
