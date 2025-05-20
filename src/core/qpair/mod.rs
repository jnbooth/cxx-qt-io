use cxx::{type_id, ExternType};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

/// Typedef for `std::pair<T::First, T::Second>`.
#[repr(C)]
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

impl<T> PartialEq for QPair<T>
where
    T: QPairPair,
    T::First: PartialEq,
    T::Second: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

impl<T> Eq for QPair<T>
where
    T: QPairPair,
    T::First: Eq,
    T::Second: Eq,
{
}

impl<T> PartialOrd for QPair<T>
where
    T: QPairPair,
    T::First: PartialOrd,
    T::Second: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.first.partial_cmp(&other.first) {
            Some(core::cmp::Ordering::Equal) => self.second.partial_cmp(&other.second),
            ord => ord,
        }
    }
}

impl<T> Ord for QPair<T>
where
    T: QPairPair,
    T::First: Ord,
    T::Second: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.first.cmp(&other.first) {
            Ordering::Equal => self.second.cmp(&other.second),
            ord => ord,
        }
    }
}

impl<T> Hash for QPair<T>
where
    T: QPairPair,
    T::First: Hash,
    T::Second: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first.hash(state);
        self.second.hash(state);
    }
}

impl<T> Debug for QPair<T>
where
    T: QPairPair,
    T::First: Debug,
    T::Second: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("QPair")
            .field("first", &self.first)
            .field("second", &self.second)
            .finish()
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

impl<T> From<(T::First, T::Second)> for QPair<T>
where
    T: QPairPair,
{
    fn from((first, second): (T::First, T::Second)) -> Self {
        Self { first, second }
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
