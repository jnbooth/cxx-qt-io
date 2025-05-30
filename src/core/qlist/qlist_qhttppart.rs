//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qhttppart.h");
        type QHttpPart = crate::QHttpPart;

        include!("cxx-qt-io/qlist_qhttppart.h");
        type QList_QHttpPart = cxx_qt_lib::QList<QHttpPart>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qlist_clear_QHttpPart"]
        fn qlistClear(list: &mut QList_QHttpPart);
        #[rust_name = "cxx_qlist_contains_QHttpPart"]
        fn qlistContains(list: &QList_QHttpPart, _: &QHttpPart) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QHttpPart"]
        fn construct(_: &QList_QHttpPart) -> QList_QHttpPart;
        #[rust_name = "qlist_default_QHttpPart"]
        fn construct() -> QList_QHttpPart;
        #[rust_name = "qlist_drop_QHttpPart"]
        fn drop(_: &mut QList_QHttpPart);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QHttpPart"]
        fn qlistReserve(_: &mut QList_QHttpPart, size: isize);
        #[rust_name = "append_QHttpPart"]
        fn qlistAppend(_: &mut QList_QHttpPart, _: &QHttpPart);
        #[rust_name = "get_unchecked_QHttpPart"]
        unsafe fn qlistGetUnchecked(set: &QList_QHttpPart, pos: isize) -> &QHttpPart;
        #[rust_name = "index_of_QHttpPart"]
        fn qlistIndexOf(_: &QList_QHttpPart, _: &QHttpPart) -> isize;
        #[rust_name = "insert_QHttpPart"]
        fn qlistInsert(_: &mut QList_QHttpPart, _: isize, _: &QHttpPart);
        #[rust_name = "remove_QHttpPart"]
        fn qlistRemove(_: &mut QList_QHttpPart, _: isize);
        #[rust_name = "len_QHttpPart"]
        fn qlistLen(_: &QList_QHttpPart) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QHttpPart) {
    ffi::cxx_qlist_clear_QHttpPart(v);
}

pub(crate) fn contains(v: &ffi::QList_QHttpPart, item: &ffi::QHttpPart) -> bool {
    ffi::cxx_qlist_contains_QHttpPart(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QHttpPart, size: isize) {
    ffi::reserve_QHttpPart(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QHttpPart, value: &ffi::QHttpPart) {
    ffi::append_QHttpPart(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QHttpPart) -> ffi::QList_QHttpPart {
    ffi::qlist_clone_QHttpPart(s)
}

pub(crate) fn default() -> ffi::QList_QHttpPart {
    ffi::qlist_default_QHttpPart()
}

pub(crate) fn drop(s: &mut ffi::QList_QHttpPart) {
    ffi::qlist_drop_QHttpPart(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QHttpPart, pos: isize) -> &ffi::QHttpPart {
    ffi::get_unchecked_QHttpPart(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QHttpPart, value: &ffi::QHttpPart) -> isize {
    ffi::index_of_QHttpPart(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QHttpPart, pos: isize, value: &ffi::QHttpPart) {
    ffi::insert_QHttpPart(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QHttpPart) -> isize {
    ffi::len_QHttpPart(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QHttpPart, pos: isize) {
    ffi::remove_QHttpPart(s, pos);
}
