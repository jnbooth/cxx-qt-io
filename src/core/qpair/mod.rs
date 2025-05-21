use cxx::{type_id, ExternType};
use std::hash::Hash;

/// Typedef for `std::pair<T1, T1>`.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QPair<T1, T2>
where
    Self: QPairPair,
{
    first: T1,
    second: T2,
}

impl<T1, T2> Drop for QPair<T1, T2>
where
    Self: QPairPair,
{
    /// Destroys the pair.
    fn drop(&mut self) {
        QPairPair::drop(self);
    }
}

impl<T1, T2> QPair<T1, T2>
where
    Self: QPairPair,
{
    pub fn first(&self) -> &T1 {
        &self.first
    }

    pub fn second(&self) -> &T2 {
        &self.second
    }
}

unsafe impl<T1, T2> ExternType for QPair<T1, T2>
where
    Self: QPairPair,
{
    type Id = <Self as QPairPair>::TypeId;
    type Kind = cxx::kind::Trivial;
}

/// Trait implementation for a pair in a [`QPair`].
pub trait QPairPair: Sized {
    type TypeId;

    fn drop(pair: &mut Self);
}

macro_rules! impl_qpair_pair {
    ( $firstTypeName:ty, $secondTypeName:ty, $module:ident, $pairTypeName:ident, $typeId:literal ) => {
        mod $module;

        impl QPairPair for QPair<$firstTypeName, $secondTypeName> {
            type TypeId = type_id!($typeId);

            fn drop(pair: &mut Self) {
                $module::drop(pair);
            }
        }
    };
}

impl_qpair_pair!(
    cxx_qt_lib::QByteArray,
    cxx_qt_lib::QByteArray,
    qpair_qbytearray_qbytearray,
    QPairPair_QByteArray_QByteArray,
    "QPair_QByteArray_QByteArray"
);

#[cfg(feature = "qt_network")]
impl_qpair_pair!(
    crate::QHostAddress,
    i32,
    qpair_qhostaddress_i32,
    QPairPair_QHostAddress_i32,
    "QPair_QHostAddress_i32"
);
