//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qsslcertificate.h");
        type QSslCertificate = crate::QSslCertificate;

        include!("cxx-qt-io/qlist_qsslcertificate.h");
        type QList_QSslCertificate = cxx_qt_lib::QList<QSslCertificate>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QSslCertificate"]
        fn qlistClear(list: &mut QList_QSslCertificate);
        #[rust_name = "qlist_contains_QSslCertificate"]
        fn qlistContains(list: &QList_QSslCertificate, _: &QSslCertificate) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSslCertificate"]
        fn construct(_: &QList_QSslCertificate) -> QList_QSslCertificate;
        #[rust_name = "qlist_default_QSslCertificate"]
        fn construct() -> QList_QSslCertificate;
        #[rust_name = "qlist_drop_QSslCertificate"]
        fn drop(_: &mut QList_QSslCertificate);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QSslCertificate"]
        fn qlistReserve(_: &mut QList_QSslCertificate, size: isize);
        #[rust_name = "qlist_append_QSslCertificate"]
        fn qlistAppend(_: &mut QList_QSslCertificate, _: &QSslCertificate);
        #[rust_name = "qlist_get_unchecked_QSslCertificate"]
        unsafe fn qlistGetUnchecked(set: &QList_QSslCertificate, pos: isize) -> &QSslCertificate;
        #[rust_name = "qlist_index_of_QSslCertificate"]
        fn qlistIndexOf(_: &QList_QSslCertificate, _: &QSslCertificate) -> isize;
        #[rust_name = "qlist_insert_QSslCertificate"]
        fn qlistInsert(_: &mut QList_QSslCertificate, _: isize, _: &QSslCertificate);
        #[rust_name = "qlist_remove_QSslCertificate"]
        fn qlistRemove(_: &mut QList_QSslCertificate, _: isize);
        #[rust_name = "qlist_len_QSslCertificate"]
        fn qlistLen(_: &QList_QSslCertificate) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslCertificate) {
    ffi::qlist_clear_QSslCertificate(v);
}

pub(crate) fn contains(v: &ffi::QList_QSslCertificate, item: &ffi::QSslCertificate) -> bool {
    ffi::qlist_contains_QSslCertificate(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslCertificate, size: isize) {
    ffi::qlist_reserve_QSslCertificate(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QSslCertificate, value: &ffi::QSslCertificate) {
    ffi::qlist_append_QSslCertificate(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QSslCertificate) -> ffi::QList_QSslCertificate {
    ffi::qlist_clone_QSslCertificate(s)
}

pub(crate) fn default() -> ffi::QList_QSslCertificate {
    ffi::qlist_default_QSslCertificate()
}

pub(crate) fn drop(s: &mut ffi::QList_QSslCertificate) {
    ffi::qlist_drop_QSslCertificate(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QSslCertificate,
    pos: isize,
) -> &ffi::QSslCertificate {
    ffi::qlist_get_unchecked_QSslCertificate(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QSslCertificate, value: &ffi::QSslCertificate) -> isize {
    ffi::qlist_index_of_QSslCertificate(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QSslCertificate, pos: isize, value: &ffi::QSslCertificate) {
    ffi::qlist_insert_QSslCertificate(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslCertificate) -> isize {
    ffi::qlist_len_QSslCertificate(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslCertificate, pos: isize) {
    ffi::qlist_remove_QSslCertificate(s, pos);
}
