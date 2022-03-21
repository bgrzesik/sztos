
pub trait DeviceRegister<T> {
    fn get_address(&self) -> usize;
}

pub trait ReadableRegister<T> : DeviceRegister<T> {
    unsafe fn get_ptr(&self) -> *const T {
        (self.get_address() as *const ()) as *const T
    }

    fn get_value(&self) -> T {
        unsafe { core::ptr::read_volatile(self.get_ptr()) }
    }
}

pub trait WritableRegister<T> : DeviceRegister<T> {
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
    (
        !device_register ro, $reg:ident, $offset:literal, $reg_ty:ty
    ) => {
        impl ReadableRegister<$reg_ty> for $reg {
        }
    };

    (
        !device_register wo, $reg:ident, $offset:literal, $reg_ty:ty
    ) => {
        impl WritableRegister<$reg_ty> for $reg {
        }
    };

    (
        !device_register rw, $reg:ident, $offset:literal, $reg_ty:ty
    ) => {
        device_register_map!( !device_register ro, $reg, $offset, $reg_ty );
        device_register_map!( !device_register wo, $reg, $offset, $reg_ty );
    };

    { 
        device $device:ident {
            $( 
                // REG @ 0xffff u64 
                $reg:ident @ $offset:literal $rw:ident $reg_ty:ty 

                // TODO(bgrzesik) add flags
                $( { } )? 
             ),*
        }
    } => {

        #[allow(non_snake_case)]
        pub mod $device {
            use $crate::drivers::*;

            #[allow(dead_code)]
            pub struct Registers(pub /* base */ usize);

            $( 
                #[allow(dead_code)]
                pub struct $reg(pub /* addr */ usize);

                impl DeviceRegister<$reg_ty> for $reg {
                    #[inline(always)]
                    fn get_address(&self) -> usize {
                        self.0
                    }
                }

                device_register_map!( !device_register $rw, $reg, $offset, $reg_ty );
             )*

            impl Registers {
                $(
                    #[allow(dead_code)]
                    pub fn $reg(&self) -> $reg {
                        $reg (self.0 + $offset)
                    }

                 )*
            }
        }
    };
}

pub mod pl011;
