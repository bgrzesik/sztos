#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(const_for)]

mod arch;
mod panic;
mod drivers;
mod platform;
mod sync;
mod register;
mod syscall;
mod memory;

use core::ptr::write_volatile;

use arch::*;

unsafe fn kernel_start() {
    loop { 
        let p1 = 0x2137_0000;
        let p2 = 0x2138_0000;
        
        //                    \n  !  o  l  l  e  H          
        let hello_le: u64 = 0x0A_21_6F_6C_6C_65_48;
        //                    \n  !  d  l  r  o  W
        let world_le: u64 = 0x0A_21_64_6C_72_6F_57;
        
        write_volatile(p1 as *mut u64, hello_le);
        write_volatile(p2 as *mut u64, world_le);
        
        // Invalidate cache entries for given adresssess
        core::arch::asm!("dc    IVAC, x0", in("x0") (p1));
        core::arch::asm!("dc    IVAC, x0", in("x0") (p2));
        Instr::dsb();
        
        MMU::swap_pages(p1, p2);

        // Instr::dsb();
        core::arch::asm!("dsb SY");

        // core::arch::asm!("svc 0");
        
        core::arch::asm!("svc 1", in("x0") (p1), in("x1") (7));
        core::arch::asm!("svc 1", in("x0") (p2), in("x1") (7));

        let s = match arch::System::exception_level() {
            arch::ExceptionLevel::User          => "Userspace\n",
            arch::ExceptionLevel::Kernel        => "Kernel\n",
            arch::ExceptionLevel::Hypervisor    => "Hypervisor\n",
            arch::ExceptionLevel::SecureMonitor => "SecureMonitor\n",
            arch::ExceptionLevel::Unknown       => "Userspace\n",
        };

        core::arch::asm!("svc 1",
            in("x0") (s.as_ptr()),
            in("x1") (s.len()));

        switch_to_userspace(userspace_start as usize as *mut (), 
                            [0; 31],
                            0x1000_0000 as *mut ());

        loop {}
    }
}

unsafe fn userspace_start() {
    // CurrentEL is not accesible at EL0, reading causes trap
    let s = "Userspace\n";
    core::arch::asm!("
        svc 1
    ",
        in("x0") (s.as_ptr()),
        in("x1") (s.len()));

    loop {}
}
