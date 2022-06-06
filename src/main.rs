#![no_std]
#![no_main]
#![feature(const_mut_refs)]

mod arch;
mod panic;
mod drivers;
mod platform;
mod sync;
mod register;
mod syscall;
mod memory;

use arch::*;

unsafe fn kernel_start() {
    loop { 
        let s = "ABCDDD\n";
        core::arch::asm!("
            svc 1
        ",
            in("x0") (s.as_ptr()),
            in("x1") (s.len()));

        let s = "12123123\n";
        core::arch::asm!("
            svc 1
        ",
            in("x0") (s.as_ptr()),
            in("x1") (s.len()));

        let s = match arch::System::exception_level() {
            arch::ExceptionLevel::User          => "Userspace\n",
            arch::ExceptionLevel::Kernel        => "Kernel\n",
            arch::ExceptionLevel::Hypervisor    => "Hypervisor\n",
            arch::ExceptionLevel::SecureMonitor => "SecureMonitor\n",
            arch::ExceptionLevel::Unknown       => "Userspace\n",
        };
        core::arch::asm!("
            svc 1
        ",
            in("x0") (s.as_ptr()),
            in("x1") (s.len()));

        switch_to_userspace(userspace_start as usize as *mut (), 
                            [0; 31],
                            0x6000_0000 as *mut ());

        loop {}
    }
}

unsafe fn userspace_start() {
    // CurrentEL is not accesible at EL0, reading causes trap
    let s = "Userspace";
    core::arch::asm!("
        svc 1
    ",
        in("x0") (s.as_ptr()),
        in("x1") (s.len()));

    loop {}
}
