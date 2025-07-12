#![allow(clippy::wildcard_imports)]
#![allow(non_camel_case_types)]

#[allow(unused)]
use cxx_qt_lib::{QHash, QHashPair};

#[allow(unused)]
unsafe fn cast<From, To>(hash: &QHash<From>) -> &QHash<To>
where
    From: QHashPair,
    To: QHashPair,
{
    // SAFETY: Provided by const assertions inside impl_qhash_pair.
    unsafe { &*(std::ptr::from_ref(hash).cast()) }
}

#[allow(unused)]
unsafe fn cast_mut<From, To>(list: &mut QHash<From>) -> &mut QHash<To>
where
    From: QHashPair,
    To: QHashPair<Value = From::Value>,
{
    // SAFETY: Provided by const assertions inside impl_qhash_pair.
    unsafe { &mut *(std::ptr::from_mut(list).cast()) }
}

#[allow(unused)]
macro_rules! impl_qhash_pair {
    ($t:ident, $typeId:literal, $k:ty, $r:ty) => {
        // Assert key size equivalency.
        const _: [(); std::mem::size_of::<$k>()] =
            [(); std::mem::size_of::<<$r as QHashPair>::Key>()];

        pub struct $t;

        impl QHashPair for $t {
            type TypeId = cxx::type_id!($typeId);
            type Key = $k;
            type Value = <$r as QHashPair>::Value;

            fn clear(hash: &mut QHash<Self>) {
                <$r as QHashPair>::clear(unsafe { cast_mut(hash) })
            }

            fn clone(hash: &QHash<Self>) -> QHash<Self> {
                unsafe { std::mem::transmute(<$r as QHashPair>::clone({ cast(hash) })) }
            }

            fn contains(hash: &QHash<Self>, k: &Self::Key) -> bool {
                <$r as QHashPair>::contains(unsafe { cast(hash) }, &k.repr)
            }

            fn default() -> QHash<Self> {
                unsafe { std::mem::transmute(<$r as QHashPair>::default()) }
            }

            fn drop(hash: &mut QHash<Self>) {
                <$r as QHashPair>::drop(unsafe { cast_mut(hash) });
            }

            fn get_or_default(hash: &QHash<Self>, key: &Self::Key) -> Self::Value {
                <$r as QHashPair>::get_or_default(unsafe { cast(hash) }, &key.repr)
            }

            unsafe fn get_unchecked_key(hash: &QHash<Self>, pos: isize) -> &Self::Key {
                unsafe {
                    &*(std::ptr::from_ref(<$r as QHashPair>::get_unchecked_key(
                        { cast(hash) },
                        pos,
                    ))
                    .cast())
                }
            }

            unsafe fn get_unchecked_value(hash: &QHash<Self>, pos: isize) -> &Self::Value {
                <$r as QHashPair>::get_unchecked_value(unsafe { cast(hash) }, pos)
            }

            fn insert(hash: &mut QHash<Self>, key: Self::Key, value: Self::Value) {
                <$r as QHashPair>::insert(unsafe { cast_mut(hash) }, key.repr, value);
            }

            fn insert_clone(hash: &mut QHash<Self>, key: &Self::Key, value: &Self::Value) {
                <$r as QHashPair>::insert_clone(unsafe { cast_mut(hash) }, &key.repr, &value);
            }

            fn len(hash: &QHash<Self>) -> isize {
                <$r as QHashPair>::len(unsafe { cast(hash) })
            }

            fn remove(hash: &mut QHash<Self>, key: &Self::Key) -> bool {
                <$r as QHashPair>::remove(unsafe { cast_mut(hash) }, &key.repr)
            }
        }
    };
}

#[cfg(feature = "request")]
impl_qhash_pair!(
    QHashPair_QNetworkRequestAttribute_QVariant,
    "QHash_QNetworkRequestAttribute_QVariant",
    crate::QNetworkRequestAttribute,
    crate::QHashPair_i32_QVariant
);
