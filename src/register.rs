pub trait TypedRegister<T>: From<T> + Into<T> {}

#[macro_export]
macro_rules! typed_register {

    ////////////////////////////////////////////////////////////////////////////////////////////

    ( !field_val $field:tt, $reg_ty:ty, $bit:literal,) => {
        ($field & (1 << $bit)) == (1 << $bit)
    };

    ( !field_val $field:tt, $reg_ty:ty, $bit:literal, $bit2:literal ) => {
        {
            let mask = (1 << ($bit - $bit2 + 1)) - 1;

            ($field & (mask << $bit2)) >> $bit2
        }
    };

    ( !field_num $field:tt, $reg_ty:ty, $bit:literal,) => {
        (if $field { 1 << $bit } else { 0 })
    };

    ( !field_num $field:tt, $reg_ty:ty, $bit:literal, $bit2:literal ) => {
        {
            let mask = (1 << ($bit - $bit2 + 1)) - 1;
            (($field & mask) << $bit2) as $reg_ty
        }
    };

    ( !field_type $reg_ty:ty, $bit:literal,) => { bool };

    ( !field_type $reg_ty:ty, $bit:literal, $bit2: literal) => { $reg_ty };

    ////////////////////////////////////////////////////////////////////////////////////////////

    (
        register $reg:ident: $reg_ty:ty
        {
            $( $field:ident @ $bit:literal $( : $bit2:literal )? ),*
        }
    ) => {

        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[derive(Default, Clone, Copy)]
        pub struct $reg {
            $(
                pub $field: $crate::typed_register!( !field_type $reg_ty, $bit, $( $bit2 )? ),
            )*
        }

        impl $reg {
            const fn value(&self) -> $reg_ty {
                0 $( | $crate::typed_register!( !field_num (self.$field), $reg_ty, $bit, $( $bit2 )? ) )*
            }
        }

        impl $crate::register::TypedRegister<$reg_ty> for $reg {
        }

        impl From<$reg> for $reg_ty {
            fn from(val: $reg) -> Self {
                val.value()
            }
        }

        impl From<$reg_ty> for $reg {
            fn from(val: $reg_ty) -> Self {
                Self {
                    $( $field: $crate::typed_register!( !field_val val, $reg_ty, $bit, $( $bit2 )? ), )*
                }
            }
        }
    };
}
