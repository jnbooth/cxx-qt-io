use cxx::{type_id, ExternType};
use cxx_qt_lib::QByteArray;
use std::hash::Hash;

/// Typedef for `std::pair<T1, T1>`.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QPair<T1, T2> {
    pub first: T1,
    pub second: T2,
}

macro_rules! impl_extern {
    ( $firstTypeName:ty, $secondTypeName:ty, $typeId:literal ) => {
        unsafe impl ExternType for QPair<$firstTypeName, $secondTypeName> {
            type Id = type_id!($typeId);
            type Kind = cxx::kind::Trivial;
        }
    };
}

impl_extern!(QByteArray, QByteArray, "QPair_QByteArray_QByteArray");

#[cfg(feature = "qt_network")]
impl_extern!(crate::QHostAddress, i32, "QPair_QHostAddress_i32");
