//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qset/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-io/qsslerror.h");
        type QSslError = crate::QSslError;

        include!("cxx-qt-io/qset_qsslerror.h");
        type QSet_QSslError = cxx_qt_lib::QSet<QSslError>;
    }

    #[namespace = "rust::cxxqtio1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_clear_QSslError"]
        fn qsetClear(set: &mut QSet_QSslError);
        #[rust_name = "qset_contains_QSslError"]
        fn qsetContains(set: &QSet_QSslError, _: &QSslError) -> bool;
        #[rust_name = "qset_remove_QSslError"]
        fn qsetRemove(set: &mut QSet_QSslError, _: &QSslError) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QSslError"]
        fn construct(_: &QSet_QSslError) -> QSet_QSslError;
        #[rust_name = "qset_default_QSslError"]
        fn construct() -> QSet_QSslError;
        #[rust_name = "qset_drop_QSslError"]
        fn drop(_: &mut QSet_QSslError);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_get_unchecked_QSslError"]
        unsafe fn qsetGetUnchecked(set: &QSet_QSslError, pos: isize) -> &QSslError;
        #[rust_name = "qset_insert_QSslError"]
        fn qsetInsert(_: &mut QSet_QSslError, _: &QSslError);
        #[rust_name = "qset_len_QSslError"]
        fn qsetLen(_: &QSet_QSslError) -> isize;
        #[rust_name = "qset_reserve_QSslError"]
        fn qsetReserve(_: &mut QSet_QSslError, size: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QSet_QSslError) {
    ffi::qset_clear_QSslError(v);
}

pub(crate) fn contains(v: &ffi::QSet_QSslError, item: &ffi::QSslError) -> bool {
    ffi::qset_contains_QSslError(v, item)
}

pub(crate) fn remove(v: &mut ffi::QSet_QSslError, item: &ffi::QSslError) -> bool {
    ffi::qset_remove_QSslError(v, item)
}

pub(crate) fn clone(s: &ffi::QSet_QSslError) -> ffi::QSet_QSslError {
    ffi::qset_clone_QSslError(s)
}

pub(crate) fn default() -> ffi::QSet_QSslError {
    ffi::qset_default_QSslError()
}

pub(crate) fn drop(s: &mut ffi::QSet_QSslError) {
    ffi::qset_drop_QSslError(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QSslError, pos: isize) -> &ffi::QSslError {
    ffi::qset_get_unchecked_QSslError(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QSslError, value: &ffi::QSslError) {
    ffi::qset_insert_QSslError(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QSslError) -> isize {
    ffi::qset_len_QSslError(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_QSslError, size: isize) {
    ffi::qset_reserve_QSslError(s, size);
}
