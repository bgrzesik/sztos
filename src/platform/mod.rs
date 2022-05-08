


#[cfg(feature="platform-rpi4")]
pub mod rpi4;
#[cfg(feature="platform-rpi4")]
pub use rpi4::*;

#[cfg(feature="platform-qemu")]
pub mod qemu;
#[cfg(feature="platform-qemu")]
pub use qemu::*;



