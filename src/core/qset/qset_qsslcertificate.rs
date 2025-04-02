//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qset/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-io/qsslcertificate.h");
        type QSslCertificate = crate::QSslCertificate;

        include!("cxx-qt-io/qset_qsslcertificate.h");
        type QSet_QSslCertificate = cxx_qt_lib::QSet<QSslCertificate>;
    }

    #[namespace = "rust::cxxqtio1::qset"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qset_clear_QSslCertificate"]
        fn qsetClear(set: &mut QSet_QSslCertificate);
        #[rust_name = "cxx_qset_contains_QSslCertificate"]
        fn qsetContains(set: &QSet_QSslCertificate, _: &QSslCertificate) -> bool;
        #[rust_name = "cxx_qset_remove_QSslCertificate"]
        fn qsetRemove(set: &QSet_QSslCertificate, _: &QSslCertificate) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QSslCertificate"]
        fn construct(_: &QSet_QSslCertificate) -> QSet_QSslCertificate;
        #[rust_name = "qset_default_QSslCertificate"]
        fn construct() -> QSet_QSslCertificate;
        #[rust_name = "qset_drop_QSslCertificate"]
        fn drop(_: &mut QSet_QSslCertificate);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_QSslCertificate"]
        unsafe fn qsetGetUnchecked(set: &QSet_QSslCertificate, pos: isize) -> &QSslCertificate;
        #[rust_name = "insert_QSslCertificate"]
        fn qsetInsert(_: &mut QSet_QSslCertificate, _: &QSslCertificate);
        #[rust_name = "len_QSslCertificate"]
        fn qsetLen(_: &QSet_QSslCertificate) -> isize;
        #[rust_name = "reserve_QSslCertificate"]
        fn qsetReserve(_: &mut QSet_QSslCertificate, size: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QSet_QSslCertificate) {
    ffi::cxx_qset_clear_QSslCertificate(v);
}

pub(crate) fn contains(v: &ffi::QSet_QSslCertificate, item: &ffi::QSslCertificate) -> bool {
    ffi::cxx_qset_contains_QSslCertificate(v, item)
}

pub(crate) fn remove(v: &ffi::QSet_QSslCertificate, item: &ffi::QSslCertificate) -> bool {
    ffi::cxx_qset_remove_QSslCertificate(v, item)
}

pub(crate) fn clone(s: &ffi::QSet_QSslCertificate) -> ffi::QSet_QSslCertificate {
    ffi::qset_clone_QSslCertificate(s)
}

pub(crate) fn default() -> ffi::QSet_QSslCertificate {
    ffi::qset_default_QSslCertificate()
}

pub(crate) fn drop(s: &mut ffi::QSet_QSslCertificate) {
    ffi::qset_drop_QSslCertificate(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QSet_QSslCertificate,
    pos: isize,
) -> &ffi::QSslCertificate {
    ffi::get_unchecked_QSslCertificate(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QSslCertificate, value: &ffi::QSslCertificate) {
    ffi::insert_QSslCertificate(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QSslCertificate) -> isize {
    ffi::len_QSslCertificate(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_QSslCertificate, size: isize) {
    ffi::reserve_QSslCertificate(s, size);
}
