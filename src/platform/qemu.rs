use crate::drivers::pl011;

use crate::sync::SpinLock;

pub const MMIO_RANGE: core::ops::Range<u64> = 0x3f201000..0x4000FFFF;

#[allow(dead_code)]
pub static mut UART0: SpinLock<pl011::Uart> = {
    let cfg = pl011::Config {
        base_addr: 0x3f201000,
        base_clk: 250_000_000,
    };

    SpinLock::new(pl011::Uart::new(
        &cfg,
        115200,
        pl011::StopBit::One,
        Some(pl011::Parity::Even),
    ))
};
