#[repr(u32)]
enum SyscallNo {
    Noop = 0,
    Print = 1,
}

impl core::convert::TryFrom<u32> for SyscallNo {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use SyscallNo::*;

        Ok(match value {
            0 => Noop,
            1 => Print,

            _ => {
                return Err(());
            }
        })
    }
}

mod print;

pub fn handle_syscall(no: u64, args: &mut [u64], ret_pc: &mut *mut ()) {
    let no = SyscallNo::try_from(no as u32);
    let no = if let Ok(no) = no { no } else { return };

    match no {
        SyscallNo::Noop => {}
        SyscallNo::Print => {
            print::handle(args, ret_pc);
        }
    }
}

extern "C" {
    #[allow(unused)]
    fn syscall(no: u64, args: ...);
}
