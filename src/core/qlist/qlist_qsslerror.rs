//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qsslerror.h");
        type QSslError = crate::QSslError;

        include!("cxx-qt-io/qlist_qsslerror.h");
        type QList_QSslError = cxx_qt_lib::QList<QSslError>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_QSslError"]
        fn qlistClear(list: &mut QList_QSslError);
        #[rust_name = "cxx_contains"]
        fn qlistContains(list: &QList_QSslError, _: &QSslError) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSslError"]
        fn construct(_: &QList_QSslError) -> QList_QSslError;
        #[rust_name = "qlist_default_QSslError"]
        fn construct() -> QList_QSslError;
        #[rust_name = "qlist_drop_QSslError"]
        fn drop(_: &mut QList_QSslError);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSslError"]
        fn qlistReserve(_: &mut QList_QSslError, size: isize);
        #[rust_name = "append_QSslError"]
        fn qlistAppend(_: &mut QList_QSslError, _: &QSslError);
        #[rust_name = "get_unchecked_QSslError"]
        unsafe fn qlistGetUnchecked(set: &QList_QSslError, pos: isize) -> &QSslError;
        #[rust_name = "index_of_QSslError"]
        fn qlistIndexOf(_: &QList_QSslError, _: &QSslError) -> isize;
        #[rust_name = "insert_QSslError"]
        fn qlistInsert(_: &mut QList_QSslError, _: isize, _: &QSslError);
        #[rust_name = "remove_QSslError"]
        fn qlistRemove(_: &mut QList_QSslError, _: isize);
        #[rust_name = "len_QSslError"]
        fn qlistLen(_: &QList_QSslError) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslError) {
    ffi::cxx_clear_qlist_QSslError(v);
}

pub(crate) fn contains(v: &ffi::QList_QSslError, item: &ffi::QSslError) -> bool {
    ffi::cxx_contains(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslError, size: isize) {
    ffi::reserve_QSslError(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QSslError, value: &ffi::QSslError) {
    ffi::append_QSslError(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QSslError) -> ffi::QList_QSslError {
    ffi::qlist_clone_QSslError(s)
}

pub(crate) fn default() -> ffi::QList_QSslError {
    ffi::qlist_default_QSslError()
}

pub(crate) fn drop(s: &mut ffi::QList_QSslError) {
    ffi::qlist_drop_QSslError(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QSslError, pos: isize) -> &ffi::QSslError {
    ffi::get_unchecked_QSslError(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QSslError, value: &ffi::QSslError) -> isize {
    ffi::index_of_QSslError(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QSslError, pos: isize, value: &ffi::QSslError) {
    ffi::insert_QSslError(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslError) -> isize {
    ffi::len_QSslError(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslError, pos: isize) {
    ffi::remove_QSslError(s, pos);
}
