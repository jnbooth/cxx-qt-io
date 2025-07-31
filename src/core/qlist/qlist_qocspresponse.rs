//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qocspresponse.h");
        type QOcspResponse = crate::QOcspResponse;

        include!("cxx-qt-io/qlist_qocspresponse.h");
        type QList_QOcspResponse = cxx_qt_lib::QList<QOcspResponse>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QOcspResponse"]
        fn qlistClear(list: &mut QList_QOcspResponse);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QOcspResponse"]
        fn construct(_: &QList_QOcspResponse) -> QList_QOcspResponse;
        #[rust_name = "qlist_default_QOcspResponse"]
        fn construct() -> QList_QOcspResponse;
        #[rust_name = "qlist_drop_QOcspResponse"]
        fn drop(_: &mut QList_QOcspResponse);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QOcspResponse"]
        fn qlistReserve(_: &mut QList_QOcspResponse, size: isize);
        #[rust_name = "qlist_append_QOcspResponse"]
        fn qlistAppend(_: &mut QList_QOcspResponse, _: &QOcspResponse);
        #[rust_name = "qlist_get_unchecked_QOcspResponse"]
        unsafe fn qlistGetUnchecked(set: &QList_QOcspResponse, pos: isize) -> &QOcspResponse;
        #[rust_name = "qlist_insert_QOcspResponse"]
        fn qlistInsert(_: &mut QList_QOcspResponse, _: isize, _: &QOcspResponse);
        #[rust_name = "qlist_remove_QOcspResponse"]
        fn qlistRemove(_: &mut QList_QOcspResponse, _: isize);
        #[rust_name = "qlist_len_QOcspResponse"]
        fn qlistLen(_: &QList_QOcspResponse) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QOcspResponse) {
    ffi::qlist_clear_QOcspResponse(v);
}

pub(crate) fn contains(_: &ffi::QList_QOcspResponse, _: &ffi::QOcspResponse) -> bool {
    false
}

pub(crate) fn reserve(v: &mut ffi::QList_QOcspResponse, size: isize) {
    ffi::qlist_reserve_QOcspResponse(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QOcspResponse, value: &ffi::QOcspResponse) {
    ffi::qlist_append_QOcspResponse(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QOcspResponse) -> ffi::QList_QOcspResponse {
    ffi::qlist_clone_QOcspResponse(s)
}

pub(crate) fn default() -> ffi::QList_QOcspResponse {
    ffi::qlist_default_QOcspResponse()
}

pub(crate) fn drop(s: &mut ffi::QList_QOcspResponse) {
    ffi::qlist_drop_QOcspResponse(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QOcspResponse,
    pos: isize,
) -> &ffi::QOcspResponse {
    unsafe { ffi::qlist_get_unchecked_QOcspResponse(s, pos) }
}

pub(crate) fn index_of(_: &ffi::QList_QOcspResponse, _: &ffi::QOcspResponse) -> isize {
    -1
}

pub(crate) fn insert(s: &mut ffi::QList_QOcspResponse, pos: isize, value: &ffi::QOcspResponse) {
    ffi::qlist_insert_QOcspResponse(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QOcspResponse) -> isize {
    ffi::qlist_len_QOcspResponse(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QOcspResponse, pos: isize) {
    ffi::qlist_remove_QOcspResponse(s, pos);
}
