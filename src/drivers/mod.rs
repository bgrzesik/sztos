use crate::register::TypedRegister;

pub trait DeviceRegister {
    fn get_address(&self) -> usize;
}

pub trait ReadableRegister<T> : DeviceRegister {
    unsafe fn get_ptr(&self) -> *const T {
        (self.get_address() as *const ()) as *const T
    }

    fn value(&self) -> T {
        unsafe { core::ptr::read_volatile(self.get_ptr()) }
    }
}

pub trait WritableRegister<T> : DeviceRegister {
    // TODO(bgrzesik) reconsider switching to mut self
    unsafe fn get_mut_ptr(&self) -> *mut T {
        (self.get_address() as *mut ()) as *mut T
    }

    fn set_value(&self, value: T)  {
        // TODO(bgrzesik) add barriers?
        unsafe { core::ptr::write_volatile(self.get_mut_ptr(), value) }
    }
}


#[macro_export]
macro_rules! device_register_map {
    ////////////////////////////////////////////////////////////////////////////////////////////

    (
        !typed_register ro, $reg:ident, $reg_ty:ty, $fields:tt
    ) => {
        impl $reg {
            #[allow(dead_code)]
            #[allow(unused_attributes)]
            pub fn typed(&self) -> super::typed::$reg {
                self.value().into()
            }
        }
    };

    (
        !typed_register wo, $reg:ident, $reg_ty:ty, $fields:tt
    ) => {
        impl $reg {
            #[allow(dead_code)]
            #[allow(unused_attributes)]
            pub fn set_typed(&self, typed: super::typed::$reg) {
                self.set_value(typed.into());
            }
        }
    };

    (
        !typed_register rw, $reg:ident, $reg_ty:ty, $fields:tt
    ) => {
        $crate::device_register_map!( !typed_register ro, $reg, $reg_ty, $fields );
        $crate::device_register_map!( !typed_register wo, $reg, $reg_ty, $fields );
    };

    ////////////////////////////////////////////////////////////////////////////////////////////

    (
        !device_register ro, $reg:ident, $offset:literal, $reg_ty:ty
    ) => {
        impl ReadableRegister<$reg_ty> for $reg {}
    };

    ////////////////////////////////////////////////////////////////////////////////////////////

    (
        !device_register wo, $reg:ident, $offset:literal, $reg_ty:ty
    ) => {
        impl WritableRegister<$reg_ty> for $reg {}
    };

    ////////////////////////////////////////////////////////////////////////////////////////////

    (
        !device_register rw, $reg:ident, $offset:literal, $reg_ty:ty
    ) => {
        $crate::device_register_map!( !device_register ro, $reg, $offset, $reg_ty );
        $crate::device_register_map!( !device_register wo, $reg, $offset, $reg_ty );
    };

    { 
        $( 
            // REG @ 0xffff u64 
            $reg:ident @ $offset:literal $rw:ident $reg_ty:ty 
            $( : $fields:tt )?
         ),*
    } => {
        use $crate::drivers::*;

        #[allow(dead_code)]
        #[allow(unused_attributes)]
        pub struct Registers(pub /* base */ usize);

        pub mod regs {
            use $crate::drivers::*;
            $( 
                #[allow(dead_code)]
                pub struct $reg(pub /* addr */ usize);

                impl DeviceRegister for $reg {
                    #[inline(always)]
                    fn get_address(&self) -> usize {
                        self.0
                    }
                }

                device_register_map!( !device_register $rw, $reg, $offset, $reg_ty );
                $(
                    $crate::device_register_map!( !typed_register $rw, $reg, $reg_ty, $fields );
                )?

            )*
        }

        pub mod typed {
            $(
                $(
                    $crate::typed_register!( register $reg: $reg_ty $fields );
                 )?
             )*
        }
        pub use typed::*;

        impl Registers {
            $(
                #[allow(dead_code)]
                #[allow(non_snake_case)]
                #[inline(always)]
                pub fn $reg(&self) -> regs::$reg {
                    regs::$reg (self.0 + $offset)
                }

             )*
        }
    };
}

pub mod pl011;
