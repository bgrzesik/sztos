use crate::device_register_map;

use super::{TypedDeviceRegister, DeviceRegister};

device_register_map! {

    DR @ 0x0000 rw u32 : {
        OE   @ 11,
        BE   @ 10,
        PE   @ 9,
        FE   @ 8,
        DATA @ 7:0
    },

    RSRECR @ 0x0004 rw u32 : {
        OE @ 3,
        BE @ 2,
        PE @ 1,
        FE @ 0
    },

    FR @ 0x0018 ro u32 : {
        RI   @ 8,
        TXFE @ 7,
        RXFF @ 6,
        TXFF @ 5,
        RXFE @ 4,
        BUSY @ 3,
        DCD  @ 2,
        DSR  @ 1,
        CTS  @ 0
    },

    ILPR @ 0x0020 rw u32,

    IBRD @ 0x0024 rw u32 : {
        IBRD  @ 15:0
    },

    FBRD @ 0x0028 rw u32 : {
        FBRD  @ 5:0
    },

    LCRH @ 0x002c rw u32 : {
        SPS  @ 7,
        WLEN @ 6:5,
        FEN  @ 4,
        STP2 @ 3,
        EPS  @ 2,
        PEN  @ 1,
        BRK  @ 0
    },

    CR @ 0x0030 rw u32 : {
        CTSEN  @ 15,
        RSTEN  @ 14,
        OUT2   @ 13,
        OUT1   @ 12,
        RTS    @ 11,
        DTR    @ 10,
        RXE    @ 9,
        TXE    @ 8,
        LBE    @ 7,
        // Reserved 6:3
        SIRLP  @ 2,
        SIREN  @ 1,
        UARTEN @ 0
    },

    IFLS @ 0x0034 rw u32 : {
        RXIFPSEL @ 11:9,
        TXIFPSEL @ 8:6,
        RXIFLSEL @ 5:3,
        TXIFFSEL @ 2:0
    },

    IMSC @ 0x0038 rw u32 : {
        OEIM   @ 10,
        BEIM   @ 9,
        PEIM   @ 8,
        FEIM   @ 7,
        RTIM   @ 6,
        TXIM   @ 5,
        RXIM   @ 4,
        DSRMIM @ 3,
        DCDMIM @ 2,
        CTSMIM @ 1,
        RIMIM  @ 0
    },

    RIS @ 0x003c rw u32 : {
        OERIS   @ 10,
        BERIS   @ 9,
        PERIS   @ 8,
        FERIS   @ 7,
        RTRIS   @ 6,
        TXRIS   @ 5,
        RXRIS   @ 4,
        DSRRMIS @ 3,
        DCDRMIS @ 2,
        CTSRMIS @ 1,
        RIRMIS  @ 0
    },

    MIS @ 0x0040 rw u32 : {
        OEMIS   @ 10,
        BEMIS   @ 9,
        PEMIS   @ 8,
        FEMIS   @ 7,
        RTMIS   @ 6,
        TXMIS   @ 5,
        RXMIS   @ 4,
        DSRRMIS @ 3,
        DCDRMIS @ 2,
        CTSRMIS @ 1,
        RIRMIS  @ 0
    },

    ICR @ 0x0044 rw u32 : {
        OEIC   @ 10,
        BEIC   @ 9,
        PEIC   @ 8,
        FEIC   @ 7,
        RTIC   @ 6,
        TXIC   @ 5,
        RXIC   @ 4,
        DSRRIC @ 3,
        DCDRIC @ 2,
        CTSRIC @ 1,
        RIRIC  @ 0
    },

    DMACR  @ 0x0048 rw u32 : {
        DMAONERR @ 2,
        TXDMAE   @ 1,
        RXDMEA   @ 0
    },

    ITCR @ 0x0080 rw u32 : {
        ITCR1 @ 1,
        ITCR0 @ 0
    },

    ITIP @ 0x0084 rw u32 : {
        ITIP3 @ 3,
        // Reserved 2:1
        ITIP0 @ 0
    },

    ITOP @ 0x0088 rw u32 : {
        ITOP11 @ 11,
        ITOP10 @ 10,
        ITOP9  @ 9,
        ITOP8  @ 8,
        ITOP7  @ 7,
        ITOP6  @ 6,
        // Reserved 5:4
        ITOP3  @ 3,
        // Reserved 2:1
        ITOP0  @ 0
    },

    TDR @ 0x008c rw u32 : {
        TDR10_0 @ 10:0
    }
}

#[allow(unused)]
pub struct Config {
    pub base_addr: usize,
    pub base_clk: u64
}

#[allow(unused)]
#[derive(PartialEq, Eq)]
pub enum Parity {
    Odd,
    Even,
}

#[allow(unused)]
#[derive(PartialEq, Eq)]
pub enum StopBit {
    One,
    Two,
}

pub struct Uart {
    reg: Registers,

    base_clk: u64,

    baudrate: u32,
    stop: StopBit,
    parity: Option<Parity>,
}

impl Uart {
    pub fn new(cfg: &Config, baudrate: u32, stop: StopBit, parity: Option<Parity>) -> Self {
        let reg = Registers(cfg.base_addr);

        Self { reg, base_clk: cfg.base_clk, baudrate, stop, parity }
    }

    pub fn reset(&mut self) {
        self.reg.CR()
            .update_typed(|cr| cr.UARTEN = false);

        self.wait_non_busy();

        let div: u32 = (4 * self.base_clk / (self.baudrate as u64)) as u32;
        let fract = div & 0b00111111;
        let int = div >> 6;

        self.reg.FBRD().set_typed(FBRD { FBRD: fract });
        self.reg.IBRD().set_typed(IBRD { IBRD: int });

        self.reg.LCRH().set_typed(LCRH::default());

        self.reg.LCRH().set_typed(LCRH {
            // Only support 8 bit
            WLEN: 0b11,
            FEN: false,
            EPS: self.parity == Some(Parity::Even),
            PEN: self.parity.is_some(),

            STP2: self.stop == StopBit::Two,

            ..Default::default()
        });

        // Mask all interupts
        self.reg.IMSC().set_value(0xffff);

        self.reg.DMACR().set_typed(DMACR::default());

        self.reg.CR()
            .update_typed(|cr| {
                cr.TXE = true;
                cr.RXE = true;
                cr.UARTEN = true;
            });
    }

    pub fn write_byte(&mut self, c: u8) {
        self.wait_non_busy();

        self.reg.DR().set_typed(typed::DR {
            DATA: c as u32,
            ..Default::default()
        });
    }

    fn wait_non_busy(&mut self) {
        while self.reg.FR().typed().BUSY {}
    }
}
