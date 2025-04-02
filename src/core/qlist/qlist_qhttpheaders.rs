//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qhttpheaders.h");
        type QHttpHeaders = crate::QHttpHeaders;

        include!("cxx-qt-io/qlist_qhttpheaders.h");
        type QList_QHttpHeaders = cxx_qt_lib::QList<QHttpHeaders>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_QHttpHeaders"]
        fn qlistClear(list: &mut QList_QHttpHeaders);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QHttpHeaders"]
        fn construct(_: &QList_QHttpHeaders) -> QList_QHttpHeaders;
        #[rust_name = "qlist_default_QHttpHeaders"]
        fn construct() -> QList_QHttpHeaders;
        #[rust_name = "qlist_drop_QHttpHeaders"]
        fn drop(_: &mut QList_QHttpHeaders);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QHttpHeaders"]
        fn qlistReserve(_: &mut QList_QHttpHeaders, size: isize);
        #[rust_name = "append_QHttpHeaders"]
        fn qlistAppend(_: &mut QList_QHttpHeaders, _: &QHttpHeaders);
        #[rust_name = "get_unchecked_QHttpHeaders"]
        unsafe fn qlistGetUnchecked(set: &QList_QHttpHeaders, pos: isize) -> &QHttpHeaders;
        #[rust_name = "insert_QHttpHeaders"]
        fn qlistInsert(_: &mut QList_QHttpHeaders, _: isize, _: &QHttpHeaders);
        #[rust_name = "remove_QHttpHeaders"]
        fn qlistRemove(_: &mut QList_QHttpHeaders, _: isize);
        #[rust_name = "len_QHttpHeaders"]
        fn qlistLen(_: &QList_QHttpHeaders) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QHttpHeaders) {
    ffi::cxx_clear_qlist_QHttpHeaders(v);
}

pub(crate) fn contains(_: &ffi::QList_QHttpHeaders, _: &ffi::QHttpHeaders) -> bool {
    false
}

pub(crate) fn reserve(v: &mut ffi::QList_QHttpHeaders, size: isize) {
    ffi::reserve_QHttpHeaders(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QHttpHeaders, value: &ffi::QHttpHeaders) {
    ffi::append_QHttpHeaders(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QHttpHeaders) -> ffi::QList_QHttpHeaders {
    ffi::qlist_clone_QHttpHeaders(s)
}

pub(crate) fn default() -> ffi::QList_QHttpHeaders {
    ffi::qlist_default_QHttpHeaders()
}

pub(crate) fn drop(s: &mut ffi::QList_QHttpHeaders) {
    ffi::qlist_drop_QHttpHeaders(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QHttpHeaders, pos: isize) -> &ffi::QHttpHeaders {
    ffi::get_unchecked_QHttpHeaders(s, pos)
}

pub(crate) fn index_of(_: &ffi::QList_QHttpHeaders, _: &ffi::QHttpHeaders) -> isize {
    -1
}

pub(crate) fn insert(s: &mut ffi::QList_QHttpHeaders, pos: isize, value: &ffi::QHttpHeaders) {
    ffi::insert_QHttpHeaders(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QHttpHeaders) -> isize {
    ffi::len_QHttpHeaders(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QHttpHeaders, pos: isize) {
    ffi::remove_QHttpHeaders(s, pos);
}
