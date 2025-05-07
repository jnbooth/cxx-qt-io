#![allow(clippy::wildcard_imports)]
#![allow(non_camel_case_types)]

#[allow(unused)]
use cxx_qt_lib::{QHash, QHashPair};

#[allow(unused)]
unsafe fn cast_enum_hash<From, To>(hash: &QHash<From>) -> &QHash<To>
where
    From: QHashPair,
    To: QHashPair,
{
    unsafe { &*(std::ptr::from_ref(hash).cast()) }
}

#[allow(unused)]
unsafe fn cast_enum_hash_mut<From, To>(list: &mut QHash<From>) -> &mut QHash<To>
where
    From: QHashPair,
    To: QHashPair,
{
    unsafe { &mut *(std::ptr::from_mut(list).cast()) }
}

macro_rules! impl_qhash_pair {
    ($t:ident, $typeId:literal, $k:ty, $v:ty, $r:ty) => {
        pub struct $t;

        impl QHashPair for $t {
            type TypeId = cxx::type_id!($typeId);
            type Key = $k;
            type Value = $v;

            fn clear(hash: &mut QHash<Self>) {
                <$r as QHashPair>::clear(unsafe { cast_enum_hash_mut(hash) })
            }

            fn clone(hash: &QHash<Self>) -> QHash<Self> {
                unsafe { std::mem::transmute(<$r as QHashPair>::clone({ cast_enum_hash(hash) })) }
            }

            fn contains(hash: &QHash<Self>, k: &$k) -> bool {
                <$r as QHashPair>::contains(unsafe { cast_enum_hash(hash) }, &k.repr)
            }

            fn default() -> QHash<Self> {
                unsafe { std::mem::transmute(<$r as QHashPair>::default()) }
            }

            fn drop(hash: &mut QHash<Self>) {
                <$r as QHashPair>::drop(unsafe { cast_enum_hash_mut(hash) });
            }

            fn get_or_default(hash: &QHash<Self>, key: &$k) -> $v {
                <$r as QHashPair>::get_or_default(unsafe { cast_enum_hash(hash) }, &key.repr)
            }

            unsafe fn get_unchecked_key(hash: &QHash<Self>, pos: isize) -> &$k {
                unsafe {
                    &*(std::ptr::from_ref(<$r as QHashPair>::get_unchecked_key(
                        { cast_enum_hash(hash) },
                        pos,
                    ))
                    .cast())
                }
            }

            unsafe fn get_unchecked_value(hash: &QHash<Self>, pos: isize) -> &$v {
                <$r as QHashPair>::get_unchecked_value(unsafe { cast_enum_hash(hash) }, pos)
            }

            fn insert(hash: &mut QHash<Self>, key: $k, value: $v) {
                <$r as QHashPair>::insert(unsafe { cast_enum_hash_mut(hash) }, key.repr, value);
            }

            fn insert_clone(hash: &mut QHash<Self>, key: &$k, value: &$v) {
                <$r as QHashPair>::insert_clone(
                    unsafe { cast_enum_hash_mut(hash) },
                    &key.repr,
                    &value,
                );
            }

            fn len(hash: &QHash<Self>) -> isize {
                <$r as QHashPair>::len(unsafe { cast_enum_hash(hash) })
            }

            fn remove(hash: &mut QHash<Self>, key: &$k) -> bool {
                <$r as QHashPair>::remove(unsafe { cast_enum_hash_mut(hash) }, &key.repr)
            }
        }
    };
}

#[cfg(feature = "qt_network")]
mod network {
    use super::*;

    impl_qhash_pair!(
        QHashPair_QNetworkRequestAttribute_QVariant,
        "QHash_QNetworkRequestAttribute_QVariant",
        crate::QNetworkRequestAttribute,
        cxx_qt_lib::QVariant,
        crate::QHashPair_i32_QVariant
    );
}

#[cfg(feature = "qt_network")]
pub use network::*;
