//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qset/generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-io/qocspresponse.h");
        type QOcspResponse = crate::QOcspResponse;

        include!("cxx-qt-io/qset_qocspresponse.h");
        type QSet_QOcspResponse = cxx_qt_lib::QSet<QOcspResponse>;
    }

    #[namespace = "rust::cxxqtio1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_clear_QOcspResponse"]
        fn qsetClear(set: &mut QSet_QOcspResponse);
        #[rust_name = "qset_contains_QOcspResponse"]
        fn qsetContains(set: &QSet_QOcspResponse, _: &QOcspResponse) -> bool;
        #[rust_name = "qset_remove_QOcspResponse"]
        fn qsetRemove(set: &mut QSet_QOcspResponse, _: &QOcspResponse) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QOcspResponse"]
        fn construct(_: &QSet_QOcspResponse) -> QSet_QOcspResponse;
        #[rust_name = "qset_default_QOcspResponse"]
        fn construct() -> QSet_QOcspResponse;
        #[rust_name = "qset_drop_QOcspResponse"]
        fn drop(_: &mut QSet_QOcspResponse);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_get_unchecked_QOcspResponse"]
        unsafe fn qsetGetUnchecked(set: &QSet_QOcspResponse, pos: isize) -> &QOcspResponse;
        #[rust_name = "qset_insert_QOcspResponse"]
        fn qsetInsert(_: &mut QSet_QOcspResponse, _: &QOcspResponse);
        #[rust_name = "qset_len_QOcspResponse"]
        fn qsetLen(_: &QSet_QOcspResponse) -> isize;
        #[rust_name = "qset_reserve_QOcspResponse"]
        fn qsetReserve(_: &mut QSet_QOcspResponse, size: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QSet_QOcspResponse) {
    ffi::qset_clear_QOcspResponse(v);
}

pub(crate) fn contains(v: &ffi::QSet_QOcspResponse, item: &ffi::QOcspResponse) -> bool {
    ffi::qset_contains_QOcspResponse(v, item)
}

pub(crate) fn remove(v: &mut ffi::QSet_QOcspResponse, item: &ffi::QOcspResponse) -> bool {
    ffi::qset_remove_QOcspResponse(v, item)
}

pub(crate) fn clone(s: &ffi::QSet_QOcspResponse) -> ffi::QSet_QOcspResponse {
    ffi::qset_clone_QOcspResponse(s)
}

pub(crate) fn default() -> ffi::QSet_QOcspResponse {
    ffi::qset_default_QOcspResponse()
}

pub(crate) fn drop(s: &mut ffi::QSet_QOcspResponse) {
    ffi::qset_drop_QOcspResponse(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_QOcspResponse, pos: isize) -> &ffi::QOcspResponse {
    ffi::qset_get_unchecked_QOcspResponse(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QOcspResponse, value: &ffi::QOcspResponse) {
    ffi::qset_insert_QOcspResponse(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QOcspResponse) -> isize {
    ffi::qset_len_QOcspResponse(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_QOcspResponse, size: isize) {
    ffi::qset_reserve_QOcspResponse(s, size);
}
