
pub trait DeviceRegister<T> {
    fn get_address(&self) -> usize;
}

pub trait ReadableRegister<T> : DeviceRegister<T> {
    unsafe fn get_ptr(&self) -> *const T {
        (self.get_address() as *const ()) as *const T
    }

    fn value(&self) -> T {
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

/*

            $reg:ident @ $offset:literal $rw:ident $reg_ty:ty 
            $(  :
                {
                    $( $field:ident @ $bit:literal $( : $bit2:literal )? ),*
                }
             )?


   */

#[macro_export]
macro_rules! device_register_map {

    ////////////////////////////////////////////////////////////////////////////////////////////

    ( !field_val $field:tt, $reg_ty:ty, $bit:literal,) => { 
        ($field & (1 << $bit)) == (1 << $bit)
    };

    ( !field_val $field:tt, $reg_ty:ty, $bit:literal, $bit2:literal ) => {
        {
            let mask = (1 << ($bit - $bit2)) - 1;

            ($field & (mask << $bit2)) >> $bit2
        }
    };

    ( !field_num $field:tt, $reg_ty:ty, $bit:literal,) => { 
        (if $field { 1 << $bit } else { 0 })
    };

    ( !field_num $field:tt, $reg_ty:ty, $bit:literal, $bit2:literal ) => {
        {
            let mask = (1 << ($bit - $bit2)) - 1;
            (($field & mask) << $bit2) as $reg_ty
        }
    };

    ( !field_type $reg_ty:ty, $bit:literal,) => { bool };

    ( !field_type $reg_ty:ty, $bit:literal, $bit2: literal) => { $reg_ty };

    ////////////////////////////////////////////////////////////////////////////////////////////

    (
        !register_fields $reg:ident, $reg_ty:ty, 
        {
            $( $field:ident @ $bit:literal $( : $bit2:literal )? ),*
        }
    ) => {

        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[derive(Default)]
        pub struct $reg {
            $(
                pub $field: device_register_map!( !field_type $reg_ty, $bit, $( $bit2 )? ),
            )*
        }

        impl From<$reg> for $reg_ty {
            fn from(val: $reg) -> Self {
                0 $( | device_register_map!( !field_num (val.$field), $reg_ty, $bit, $( $bit2 )? ) )*
            }
        }

        impl From<$reg_ty> for $reg {
            fn from(val: $reg_ty) -> Self {
                Self {
                    $( $field: device_register_map!( !field_val val, $reg_ty, $bit, $( $bit2 )? ), )*
                }
            }
        }
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
        device_register_map!( !device_register ro, $reg, $offset, $reg_ty );
        device_register_map!( !device_register wo, $reg, $offset, $reg_ty );
    };

    { 
        $( 
            // REG @ 0xffff u64 
            $reg:ident @ $offset:literal $rw:ident $reg_ty:ty 
            $( : $fields:tt )?

            // TODO(bgrzesik) add flags
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

                impl DeviceRegister<$reg_ty> for $reg {
                    #[inline(always)]
                    fn get_address(&self) -> usize {
                        self.0
                    }
                }

                device_register_map!( !device_register $rw, $reg, $offset, $reg_ty );

            )*
        }

        $(
            $(
                device_register_map!( !register_fields $reg, $reg_ty, $fields );
             )?
         )*

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
