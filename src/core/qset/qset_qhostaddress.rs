//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qset/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddress = crate::QHostAddress;

        include!("cxx-qt-io/qset_qhostaddress.h");
        type QSet_QHostAddress = cxx_qt_lib::QSet<QHostAddress>;
    }

    #[namespace = "rust::cxxqtio1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_clear_QHostAddress"]
        fn qsetClear(set: &mut QSet_QHostAddress);
        #[rust_name = "qset_contains_QHostAddress"]
        fn qsetContains(set: &QSet_QHostAddress, _: &QHostAddress) -> bool;
        #[rust_name = "qset_remove_QHostAddress"]
        fn qsetRemove(set: &mut QSet_QHostAddress, _: &QHostAddress) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QHostAddress"]
        fn construct(_: &QSet_QHostAddress) -> QSet_QHostAddress;
        #[rust_name = "qset_default_QHostAddress"]
        fn construct() -> QSet_QHostAddress;
        #[rust_name = "qset_drop_QHostAddress"]
        fn drop(_: &mut QSet_QHostAddress);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_get_unchecked_QHostAddress"]
        unsafe fn qsetGetUnchecked(set: &QSet_QHostAddress, pos: isize) -> &QHostAddress;
        #[rust_name = "qset_insert_QHostAddress"]
        fn qsetInsert(_: &mut QSet_QHostAddress, _: &QHostAddress);
        #[rust_name = "qset_len_QHostAddress"]
        fn qsetLen(_: &QSet_QHostAddress) -> isize;
        #[rust_name = "qset_reserve_QHostAddress"]
        fn qsetReserve(_: &mut QSet_QHostAddress, size: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QSet_QHostAddress) {
    ffi::qset_clear_QHostAddress(v);
}

pub(crate) fn contains(v: &ffi::QSet_QHostAddress, item: &ffi::QHostAddress) -> bool {
    ffi::qset_contains_QHostAddress(v, item)
}

pub(crate) fn remove(v: &mut ffi::QSet_QHostAddress, item: &ffi::QHostAddress) -> bool {
    ffi::qset_remove_QHostAddress(v, item)
}

pub(crate) fn clone(s: &ffi::QSet_QHostAddress) -> ffi::QSet_QHostAddress {
    ffi::qset_clone_QHostAddress(s)
}

pub(crate) fn default() -> ffi::QSet_QHostAddress {
    ffi::qset_default_QHostAddress()
}

pub(crate) fn drop(s: &mut ffi::QSet_QHostAddress) {
    ffi::qset_drop_QHostAddress(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QHostAddress, pos: isize) -> &ffi::QHostAddress {
    ffi::qset_get_unchecked_QHostAddress(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QHostAddress, value: &ffi::QHostAddress) {
    ffi::qset_insert_QHostAddress(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QHostAddress) -> isize {
    ffi::qset_len_QHostAddress(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_QHostAddress, size: isize) {
    ffi::qset_reserve_QHostAddress(s, size);
}
