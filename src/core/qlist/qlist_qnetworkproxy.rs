//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qnetworkproxy.h");
        type QNetworkProxy = crate::QNetworkProxy;

        include!("cxx-qt-io/qlist_qnetworkproxy.h");
        type QList_QNetworkProxy = cxx_qt_lib::QList<QNetworkProxy>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qlist_clear_QNetworkProxy"]
        fn qlistClear(list: &mut QList_QNetworkProxy);
        #[rust_name = "cxx_qlist_contains"]
        fn qlistContains(list: &QList_QNetworkProxy, _: &QNetworkProxy) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QNetworkProxy"]
        fn construct(_: &QList_QNetworkProxy) -> QList_QNetworkProxy;
        #[rust_name = "qlist_default_QNetworkProxy"]
        fn construct() -> QList_QNetworkProxy;
        #[rust_name = "qlist_drop_QNetworkProxy"]
        fn drop(_: &mut QList_QNetworkProxy);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QNetworkProxy"]
        fn qlistReserve(_: &mut QList_QNetworkProxy, size: isize);
        #[rust_name = "append_QNetworkProxy"]
        fn qlistAppend(_: &mut QList_QNetworkProxy, _: &QNetworkProxy);
        #[rust_name = "get_unchecked_QNetworkProxy"]
        unsafe fn qlistGetUnchecked(set: &QList_QNetworkProxy, pos: isize) -> &QNetworkProxy;
        #[rust_name = "index_of_QNetworkProxy"]
        fn qlistIndexOf(_: &QList_QNetworkProxy, _: &QNetworkProxy) -> isize;
        #[rust_name = "insert_QNetworkProxy"]
        fn qlistInsert(_: &mut QList_QNetworkProxy, _: isize, _: &QNetworkProxy);
        #[rust_name = "remove_QNetworkProxy"]
        fn qlistRemove(_: &mut QList_QNetworkProxy, _: isize);
        #[rust_name = "len_QNetworkProxy"]
        fn qlistLen(_: &QList_QNetworkProxy) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QNetworkProxy) {
    ffi::cxx_qlist_clear_QNetworkProxy(v);
}

pub(crate) fn contains(v: &ffi::QList_QNetworkProxy, item: &ffi::QNetworkProxy) -> bool {
    ffi::cxx_qlist_contains(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QNetworkProxy, size: isize) {
    ffi::reserve_QNetworkProxy(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QNetworkProxy, value: &ffi::QNetworkProxy) {
    ffi::append_QNetworkProxy(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QNetworkProxy) -> ffi::QList_QNetworkProxy {
    ffi::qlist_clone_QNetworkProxy(s)
}

pub(crate) fn default() -> ffi::QList_QNetworkProxy {
    ffi::qlist_default_QNetworkProxy()
}

pub(crate) fn drop(s: &mut ffi::QList_QNetworkProxy) {
    ffi::qlist_drop_QNetworkProxy(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QNetworkProxy,
    pos: isize,
) -> &ffi::QNetworkProxy {
    ffi::get_unchecked_QNetworkProxy(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QNetworkProxy, value: &ffi::QNetworkProxy) -> isize {
    ffi::index_of_QNetworkProxy(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QNetworkProxy, pos: isize, value: &ffi::QNetworkProxy) {
    ffi::insert_QNetworkProxy(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QNetworkProxy) -> isize {
    ffi::len_QNetworkProxy(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QNetworkProxy, pos: isize) {
    ffi::remove_QNetworkProxy(s, pos);
}
