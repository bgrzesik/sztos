use crate::register::TypedRegister;

pub trait ReadableRegister {
}

pub trait WritableRegister {
}

pub trait DeviceRegister<T> {
    fn get_address(&self) -> usize;

    unsafe fn get_ptr(&self) -> *const T
        where Self: ReadableRegister
    {
        (self.get_address() as *const ()) as *const T
    }

    fn value(&self) -> T
        where Self: ReadableRegister
    {
        unsafe { core::ptr::read_volatile(self.get_ptr()) }
    }

    // TODO(bgrzesik) reconsider switching to mut self
    unsafe fn get_mut_ptr(&self) -> *mut T
        where Self: WritableRegister
    {
        (self.get_address() as *mut ()) as *mut T
    }

    fn set_value(&self, value: T)
        where Self: WritableRegister
    {
        // TODO(bgrzesik) add barriers?
        unsafe { core::ptr::write_volatile(self.get_mut_ptr(), value) }
    }
}

pub trait TypedDeviceRegister<T, R: TypedRegister<T>>: DeviceRegister<T> {

    fn typed(&self) -> R
        where Self: ReadableRegister
    {
        R::from(self.value())
    }

    fn set_typed(&self, value: R)
        where Self: WritableRegister
    {
        self.set_value(value.into());
    }

    fn update_typed<F, Z>(&self, update_fn: F) -> Z
        where Self: ReadableRegister + WritableRegister,
              F: FnOnce(&R) -> Z
    {
        let mut value: R = self.typed();
        let ret: Z = update_fn(&mut value);
        self.set_typed(value);

        return ret;
    }

}

#[macro_export]
macro_rules! device_register_map {
    ////////////////////////////////////////////////////////////////////////////////////////////

    (
        !noop: $fields:tt
    ) => {};

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
        !device_register ro, $reg:ident
    ) => {
        impl ReadableRegister for $reg {}
    };

    ////////////////////////////////////////////////////////////////////////////////////////////

    (
        !device_register wo, $reg:ident
    ) => {
        impl WritableRegister for $reg {}
    };

    ////////////////////////////////////////////////////////////////////////////////////////////

    (
        !device_register rw, $reg:ident
    ) => {
        $crate::device_register_map!( !device_register ro, $reg );
        $crate::device_register_map!( !device_register wo, $reg );
    };

    { 
        $( 
            // REG @ 0xffff u64 
            $reg:ident @ $offset:literal $rw:ident $reg_ty:ty 
            $( : $fields:tt )?
         ),*
    } => {
        #[allow(dead_code)]
        #[allow(unused_attributes)]
        pub struct Registers(pub /* base */ usize);

        pub mod regs {
            use $crate::drivers::*;
            $( 
                #[allow(dead_code)]
                pub struct $reg(pub /* addr */ usize);

                impl DeviceRegister<$reg_ty> for $reg {
                    #[inline(always)]
                    fn get_address(&self) -> usize {
                        self.0
                    }
                }

                device_register_map!( !device_register $rw, $reg );
                $(
                    // This noop so code only spawns when $fields is present
                    const _: &str = core::stringify!($rw, $reg, $reg_ty, $fields);

                    impl TypedDeviceRegister<$reg_ty, super::typed::$reg> for $reg {}
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
