use crate::drivers::pl011;

#[allow(dead_code)]
pub const UART0: pl011::Config = pl011::Config {
    base_addr: 0x0900_0000,
    base_clk: 250_000_000
};
