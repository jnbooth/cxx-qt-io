//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qset/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-io/qhttp1configuration.h");
        type QHttp1Configuration = crate::QHttp1Configuration;

        include!("cxx-qt-io/qset_qhttp1configuration.h");
        type QSet_QHttp1Configuration = cxx_qt_lib::QSet<QHttp1Configuration>;
    }

    #[namespace = "rust::cxxqtio1::qset"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qset_clear_QHttp1Configuration"]
        fn qsetClear(set: &mut QSet_QHttp1Configuration);
        #[rust_name = "cxx_qset_contains_QHttp1Configuration"]
        fn qsetContains(set: &QSet_QHttp1Configuration, _: &QHttp1Configuration) -> bool;
        #[rust_name = "cxx_qset_remove_QHttp1Configuration"]
        fn qsetRemove(set: &mut QSet_QHttp1Configuration, _: &QHttp1Configuration) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QHttp1Configuration"]
        fn construct(_: &QSet_QHttp1Configuration) -> QSet_QHttp1Configuration;
        #[rust_name = "qset_default_QHttp1Configuration"]
        fn construct() -> QSet_QHttp1Configuration;
        #[rust_name = "qset_drop_QHttp1Configuration"]
        fn drop(_: &mut QSet_QHttp1Configuration);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_QHttp1Configuration"]
        unsafe fn qsetGetUnchecked(
            set: &QSet_QHttp1Configuration,
            pos: isize,
        ) -> &QHttp1Configuration;
        #[rust_name = "insert_QHttp1Configuration"]
        fn qsetInsert(_: &mut QSet_QHttp1Configuration, _: &QHttp1Configuration);
        #[rust_name = "len_QHttp1Configuration"]
        fn qsetLen(_: &QSet_QHttp1Configuration) -> isize;
        #[rust_name = "reserve_QHttp1Configuration"]
        fn qsetReserve(_: &mut QSet_QHttp1Configuration, size: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QSet_QHttp1Configuration) {
    ffi::cxx_qset_clear_QHttp1Configuration(v);
}

pub(crate) fn contains(v: &ffi::QSet_QHttp1Configuration, item: &ffi::QHttp1Configuration) -> bool {
    ffi::cxx_qset_contains_QHttp1Configuration(v, item)
}

pub(crate) fn remove(
    v: &mut ffi::QSet_QHttp1Configuration,
    item: &ffi::QHttp1Configuration,
) -> bool {
    ffi::cxx_qset_remove_QHttp1Configuration(v, item)
}

pub(crate) fn clone(s: &ffi::QSet_QHttp1Configuration) -> ffi::QSet_QHttp1Configuration {
    ffi::qset_clone_QHttp1Configuration(s)
}

pub(crate) fn default() -> ffi::QSet_QHttp1Configuration {
    ffi::qset_default_QHttp1Configuration()
}

pub(crate) fn drop(s: &mut ffi::QSet_QHttp1Configuration) {
    ffi::qset_drop_QHttp1Configuration(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QSet_QHttp1Configuration,
    pos: isize,
) -> &ffi::QHttp1Configuration {
    ffi::get_unchecked_QHttp1Configuration(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QHttp1Configuration, value: &ffi::QHttp1Configuration) {
    ffi::insert_QHttp1Configuration(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QHttp1Configuration) -> isize {
    ffi::len_QHttp1Configuration(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_QHttp1Configuration, size: isize) {
    ffi::reserve_QHttp1Configuration(s, size);
}
