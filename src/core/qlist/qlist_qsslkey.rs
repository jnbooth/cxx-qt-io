//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qsslkey.h");
        type QSslKey = crate::QSslKey;

        include!("cxx-qt-io/qlist_qsslkey.h");
        type QList_QSslKey = cxx_qt_lib::QList<QSslKey>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qlist_clear_QSslKey"]
        fn qlistClear(list: &mut QList_QSslKey);
        #[rust_name = "cxx_qlist_contains_QSslKey"]
        fn qlistContains(list: &QList_QSslKey, _: &QSslKey) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSslKey"]
        fn construct(_: &QList_QSslKey) -> QList_QSslKey;
        #[rust_name = "qlist_default_QSslKey"]
        fn construct() -> QList_QSslKey;
        #[rust_name = "qlist_drop_QSslKey"]
        fn drop(_: &mut QList_QSslKey);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSslKey"]
        fn qlistReserve(_: &mut QList_QSslKey, size: isize);
        #[rust_name = "append_QSslKey"]
        fn qlistAppend(_: &mut QList_QSslKey, _: &QSslKey);
        #[rust_name = "get_unchecked_QSslKey"]
        unsafe fn qlistGetUnchecked(set: &QList_QSslKey, pos: isize) -> &QSslKey;
        #[rust_name = "index_of_QSslKey"]
        fn qlistIndexOf(_: &QList_QSslKey, _: &QSslKey) -> isize;
        #[rust_name = "insert_QSslKey"]
        fn qlistInsert(_: &mut QList_QSslKey, _: isize, _: &QSslKey);
        #[rust_name = "remove_QSslKey"]
        fn qlistRemove(_: &mut QList_QSslKey, _: isize);
        #[rust_name = "len_QSslKey"]
        fn qlistLen(_: &QList_QSslKey) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslKey) {
    ffi::cxx_qlist_clear_QSslKey(v);
}

pub(crate) fn contains(v: &ffi::QList_QSslKey, item: &ffi::QSslKey) -> bool {
    ffi::cxx_qlist_contains_QSslKey(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslKey, size: isize) {
    ffi::reserve_QSslKey(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QSslKey, value: &ffi::QSslKey) {
    ffi::append_QSslKey(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QSslKey) -> ffi::QList_QSslKey {
    ffi::qlist_clone_QSslKey(s)
}

pub(crate) fn default() -> ffi::QList_QSslKey {
    ffi::qlist_default_QSslKey()
}

pub(crate) fn drop(s: &mut ffi::QList_QSslKey) {
    ffi::qlist_drop_QSslKey(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QSslKey, pos: isize) -> &ffi::QSslKey {
    ffi::get_unchecked_QSslKey(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QSslKey, value: &ffi::QSslKey) -> isize {
    ffi::index_of_QSslKey(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QSslKey, pos: isize, value: &ffi::QSslKey) {
    ffi::insert_QSslKey(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslKey) -> isize {
    ffi::len_QSslKey(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslKey, pos: isize) {
    ffi::remove_QSslKey(s, pos);
}
