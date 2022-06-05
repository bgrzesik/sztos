
use core::fmt::Write;
use crate::platform::*;

pub fn handle(args: &mut [u64], ret_pc: &mut *mut ()) {
    let uart = unsafe { &mut *UART0.lock() };

    unsafe {
        let ptr = args[0] as *const u8;
        let len = args[1] as usize;

        let slice = core::slice::from_raw_parts(ptr, len);
        let s = core::str::from_utf8_unchecked(slice);

        uart.write_str(s);
    }
}
