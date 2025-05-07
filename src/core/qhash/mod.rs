mod enums;
pub use enums::*;

use cxx::type_id;
use cxx_qt_lib::{QHash, QHashPair};

macro_rules! impl_qhash_pair {
    ( $keyTypeName:ty, $valueTypeName:ty, $module:ident, $pairTypeName:ident, $typeId:literal ) => {
        mod $module;
        pub use $module::$pairTypeName;

        impl QHashPair for $module::$pairTypeName {
            type Key = $keyTypeName;
            type Value = $valueTypeName;
            type TypeId = type_id!($typeId);

            fn clear(hash: &mut QHash<Self>) {
                $module::clear(hash);
            }

            fn clone(hash: &QHash<Self>) -> QHash<Self> {
                $module::clone(hash)
            }

            fn contains(hash: &QHash<Self>, key: &$keyTypeName) -> bool {
                $module::contains(hash, key)
            }

            fn default() -> QHash<Self> {
                $module::default()
            }

            fn drop(hash: &mut QHash<Self>) {
                $module::drop(hash);
            }

            fn get_or_default(hash: &QHash<Self>, key: &$keyTypeName) -> $valueTypeName {
                $module::get_or_default(hash, key)
            }

            unsafe fn get_unchecked_key(hash: &QHash<Self>, pos: isize) -> &$keyTypeName {
                $module::get_unchecked_key(hash, pos)
            }

            unsafe fn get_unchecked_value(hash: &QHash<Self>, pos: isize) -> &$valueTypeName {
                $module::get_unchecked_value(hash, pos)
            }

            fn insert(hash: &mut QHash<Self>, key: $keyTypeName, value: $valueTypeName) {
                $module::insert(hash, &key, &value);
            }

            fn insert_clone(hash: &mut QHash<Self>, key: &$keyTypeName, value: &$valueTypeName) {
                $module::insert(hash, key, value);
            }

            fn len(hash: &QHash<Self>) -> isize {
                $module::len(hash)
            }

            fn remove(hash: &mut QHash<Self>, key: &$keyTypeName) -> bool {
                $module::remove(hash, key)
            }
        }
    };
}

impl_qhash_pair!(
    i32,
    cxx_qt_lib::QVariant,
    qhash_i32_qvariant,
    QHashPair_i32_QVariant,
    "QHash_i32_QVariant"
);
