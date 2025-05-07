//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qhash/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qhash_i32_QVariant.h");
        type QHash_i32_QVariant = cxx_qt_lib::QHash<super::QHashPair_i32_QVariant>;
    }

    #[namespace = "rust::cxxqtio1::qhash"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qhash_clear_i32_QVariant"]
        fn qhashClear(hash: &mut QHash_i32_QVariant);
        #[rust_name = "cxx_qhash_contains_i32_QVariant"]
        fn qhashContains(hash: &QHash_i32_QVariant, _: &i32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhash_clone_i32_QVariant"]
        fn construct(_: &QHash_i32_QVariant) -> QHash_i32_QVariant;
        #[rust_name = "qhash_default_i32_QVariant"]
        fn construct() -> QHash_i32_QVariant;
        #[rust_name = "qhash_drop_i32_QVariant"]
        fn drop(_: &mut QHash_i32_QVariant);
    }

    #[namespace = "rust::cxxqtlib1::qhash"]
    unsafe extern "C++" {
        #[rust_name = "get_or_default_i32_QVariant"]
        fn qhashGetOrDefault(_: &QHash_i32_QVariant, key: &i32) -> QVariant;
        #[rust_name = "get_unchecked_key_i32_QVariant"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qhashGetUncheckedKey<'a>(_: &'a QHash_i32_QVariant, pos: isize) -> &'a i32;
        #[rust_name = "get_unchecked_value_i32_QVariant"]
        unsafe fn qhashGetUncheckedValue(_: &QHash_i32_QVariant, pos: isize) -> &QVariant;
        #[rust_name = "insert_i32_QVariant"]
        fn qhashInsert(_: &mut QHash_i32_QVariant, key: &i32, value: &QVariant);
        #[rust_name = "len_i32_QVariant"]
        fn qhashLen(_: &QHash_i32_QVariant) -> isize;
        #[rust_name = "remove_i32_QVariant"]
        fn qhashRemove(_: &mut QHash_i32_QVariant, key: &i32) -> bool;
    }
}

pub(crate) fn clear(v: &mut ffi::QHash_i32_QVariant) {
    ffi::cxx_qhash_clear_i32_QVariant(v);
}

pub(crate) fn contains(hash: &ffi::QHash_i32_QVariant, k: &i32) -> bool {
    ffi::cxx_qhash_contains_i32_QVariant(hash, k)
}

pub(crate) fn clone(hash: &ffi::QHash_i32_QVariant) -> ffi::QHash_i32_QVariant {
    ffi::qhash_clone_i32_QVariant(hash)
}

pub(crate) fn default() -> ffi::QHash_i32_QVariant {
    ffi::qhash_default_i32_QVariant()
}

pub(crate) fn drop(hash: &mut ffi::QHash_i32_QVariant) {
    ffi::qhash_drop_i32_QVariant(hash);
}

pub(crate) fn get_or_default(hash: &ffi::QHash_i32_QVariant, key: &i32) -> ffi::QVariant {
    ffi::get_or_default_i32_QVariant(hash, key)
}

pub(crate) unsafe fn get_unchecked_key(hash: &ffi::QHash_i32_QVariant, pos: isize) -> &i32 {
    ffi::get_unchecked_key_i32_QVariant(hash, pos)
}

pub(crate) unsafe fn get_unchecked_value(
    hash: &ffi::QHash_i32_QVariant,
    pos: isize,
) -> &ffi::QVariant {
    ffi::get_unchecked_value_i32_QVariant(hash, pos)
}

pub(crate) fn insert(hash: &mut ffi::QHash_i32_QVariant, key: &i32, value: &ffi::QVariant) {
    ffi::insert_i32_QVariant(hash, key, value);
}

pub(crate) fn len(hash: &ffi::QHash_i32_QVariant) -> isize {
    ffi::len_i32_QVariant(hash)
}

pub(crate) fn remove(hash: &mut ffi::QHash_i32_QVariant, key: &i32) -> bool {
    ffi::remove_i32_QVariant(hash, key)
}

#[allow(non_camel_case_types)]
pub struct QHashPair_i32_QVariant;
