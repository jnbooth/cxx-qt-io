use cxx::{type_id, ExternType};

/// Typedef for `std::pair<T::First, T::Second>`.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QPair<T>
where
    T: QPairPair,
{
    first: T::First,
    second: T::Second,
}

impl<T> Drop for QPair<T>
where
    T: QPairPair,
{
    /// Destroys the pair.
    fn drop(&mut self) {
        T::drop(self);
    }
}

impl<T> QPair<T>
where
    T: QPairPair,
{
    pub fn first(&self) -> &T::First {
        &self.first
    }

    pub fn second(&self) -> &T::Second {
        &self.second
    }
}

unsafe impl<T> ExternType for QPair<T>
where
    T: QPairPair,
{
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

/// Trait implementation for a pair in a [`QPair`].
pub trait QPairPair: Sized {
    type First;
    type Second;
    type TypeId;

    fn drop(pair: &mut QPair<Self>);
}

macro_rules! impl_qpair_pair {
    ( $firstTypeName:ty, $secondTypeName:ty, $module:ident, $pairTypeName:ident, $typeId:literal ) => {
        mod $module;
        pub use $module::$pairTypeName;

        impl QPairPair for $module::$pairTypeName {
            type First = $firstTypeName;
            type Second = $secondTypeName;
            type TypeId = type_id!($typeId);

            fn drop(pair: &mut QPair<Self>) {
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
