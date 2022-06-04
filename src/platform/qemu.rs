use crate::drivers::pl011;

use crate::sync::SpinLock;

#[allow(dead_code)]
pub static mut UART0: SpinLock<pl011::Uart> = {
    let cfg = pl011::Config {
        base_addr: 0x0900_0000,
        base_clk: 250_000_000
    };

    SpinLock::new(
        pl011::Uart::new(&cfg,
                         115200,
                         pl011::StopBit::One,
                         Some(pl011::Parity::Even))
    )
};
