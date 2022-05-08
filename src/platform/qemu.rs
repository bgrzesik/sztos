use crate::drivers::pl011::PL011;

#[allow(dead_code)]
pub static UART0: PL011::Registers = PL011::Registers(0x0900_0000);
