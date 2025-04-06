//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qnetworkrequest.h");
        type QNetworkRequest = crate::QNetworkRequest;

        include!("cxx-qt-io/qlist_qnetworkrequest.h");
        type QList_QNetworkRequest = cxx_qt_lib::QList<QNetworkRequest>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qlist_clear_QNetworkRequest"]
        fn qlistClear(list: &mut QList_QNetworkRequest);
        #[rust_name = "cxx_qlist_contains_QNetworkRequest"]
        fn qlistContains(list: &QList_QNetworkRequest, _: &QNetworkRequest) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QNetworkRequest"]
        fn construct(_: &QList_QNetworkRequest) -> QList_QNetworkRequest;
        #[rust_name = "qlist_default_QNetworkRequest"]
        fn construct() -> QList_QNetworkRequest;
        #[rust_name = "qlist_drop_QNetworkRequest"]
        fn drop(_: &mut QList_QNetworkRequest);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QNetworkRequest"]
        fn qlistReserve(_: &mut QList_QNetworkRequest, size: isize);
        #[rust_name = "append_QNetworkRequest"]
        fn qlistAppend(_: &mut QList_QNetworkRequest, _: &QNetworkRequest);
        #[rust_name = "get_unchecked_QNetworkRequest"]
        unsafe fn qlistGetUnchecked(set: &QList_QNetworkRequest, pos: isize) -> &QNetworkRequest;
        #[rust_name = "index_of_QNetworkRequest"]
        fn qlistIndexOf(_: &QList_QNetworkRequest, _: &QNetworkRequest) -> isize;
        #[rust_name = "insert_QNetworkRequest"]
        fn qlistInsert(_: &mut QList_QNetworkRequest, _: isize, _: &QNetworkRequest);
        #[rust_name = "remove_QNetworkRequest"]
        fn qlistRemove(_: &mut QList_QNetworkRequest, _: isize);
        #[rust_name = "len_QNetworkRequest"]
        fn qlistLen(_: &QList_QNetworkRequest) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QNetworkRequest) {
    ffi::cxx_qlist_clear_QNetworkRequest(v);
}

pub(crate) fn contains(v: &ffi::QList_QNetworkRequest, item: &ffi::QNetworkRequest) -> bool {
    ffi::cxx_qlist_contains_QNetworkRequest(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QNetworkRequest, size: isize) {
    ffi::reserve_QNetworkRequest(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QNetworkRequest, value: &ffi::QNetworkRequest) {
    ffi::append_QNetworkRequest(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QNetworkRequest) -> ffi::QList_QNetworkRequest {
    ffi::qlist_clone_QNetworkRequest(s)
}

pub(crate) fn default() -> ffi::QList_QNetworkRequest {
    ffi::qlist_default_QNetworkRequest()
}

pub(crate) fn drop(s: &mut ffi::QList_QNetworkRequest) {
    ffi::qlist_drop_QNetworkRequest(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QNetworkRequest,
    pos: isize,
) -> &ffi::QNetworkRequest {
    ffi::get_unchecked_QNetworkRequest(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QNetworkRequest, value: &ffi::QNetworkRequest) -> isize {
    ffi::index_of_QNetworkRequest(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QNetworkRequest, pos: isize, value: &ffi::QNetworkRequest) {
    ffi::insert_QNetworkRequest(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QNetworkRequest) -> isize {
    ffi::len_QNetworkRequest(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QNetworkRequest, pos: isize) {
    ffi::remove_QNetworkRequest(s, pos);
}
