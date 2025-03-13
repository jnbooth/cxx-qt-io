macro_rules! impl_qflag {
    ( $typeName:ty, $flagsName:ident, $typeId:literal ) => {
        pub type $flagsName = $crate::QFlags<$typeName>;

        unsafe impl $crate::QFlag for $typeName {
            type TypeId = cxx::type_id!($typeId);
            type Repr = i32;

            fn to_repr(self) -> Self::Repr {
                self.repr
            }
        }

        impl std::ops::BitOr for $typeName {
            type Output = crate::QFlags<$typeName>;

            fn bitor(self, other: Self) -> Self::Output {
                crate::QFlags::from(self) | other
            }
        }
    };
}

mod adapter;

mod core;
pub use crate::core::*;

#[cfg(feature = "qt_network")]
mod network;
#[cfg(feature = "qt_network")]
pub use crate::network::*;
