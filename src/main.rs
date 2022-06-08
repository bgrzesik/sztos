#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(const_for)]

mod arch;
mod drivers;
mod panic;
mod platform;
mod register;
mod sync;
mod syscall;

use core::ptr::write_volatile;

use arch::*;

unsafe fn kernel_start() {
    loop {
        let p1 = 0x2137_0000;
        let p2 = 0x2138_0000;

        let hello_le = b"Hello!\n";
        let world_le = b"World!\n";

        write_volatile(p1 as *mut [u8; 7], *hello_le);
        write_volatile(p2 as *mut [u8; 7], *world_le);

        core::arch::asm!("dc    IVAC, x0", in("x0") (p1));
        core::arch::asm!("dc    IVAC, x0", in("x0") (p2));

        MMU::swap_pages(p1, p2);

        core::arch::asm!("svc 0");
        core::arch::asm!("svc 1", in("x0") (p1), in("x1") (hello_le.len()));
        core::arch::asm!("svc 1", in("x0") (p2), in("x1") (world_le.len()));

        let s = match arch::System::exception_level() {
            arch::ExceptionLevel::User => "Userspace\n",
            arch::ExceptionLevel::Kernel => "Kernel\n",
            arch::ExceptionLevel::Hypervisor => "Hypervisor\n",
            arch::ExceptionLevel::SecureMonitor => "SecureMonitor\n",
            arch::ExceptionLevel::Unknown => "Userspace\n",
        };

        core::arch::asm!("svc 1",
            in("x0") (s.as_ptr()),
            in("x1") (s.len()));

        switch_to_userspace(
            userspace_start as usize as *mut (),
            [0; 31],
            0x3000_8000 as *mut (),
        );

        loop {
            Instr::wfe();
        }
    }
}

unsafe fn userspace_start() {
    // CurrentEL is not accesible at EL0, reading causes trap
    let s = "Userspace\n";
    core::arch::asm!("svc 1", in("x0") (s.as_ptr()), in("x1") (s.len()));

    loop {}
}
