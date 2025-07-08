//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qsslconfiguration.h");
        type QSslConfiguration = crate::QSslConfiguration;

        include!("cxx-qt-io/qlist_qsslconfiguration.h");
        type QList_QSslConfiguration = cxx_qt_lib::QList<QSslConfiguration>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QSslConfiguration"]
        fn qlistClear(list: &mut QList_QSslConfiguration);
        #[rust_name = "qlist_contains_QSslConfiguration"]
        fn qlistContains(list: &QList_QSslConfiguration, _: &QSslConfiguration) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSslConfiguration"]
        fn construct(_: &QList_QSslConfiguration) -> QList_QSslConfiguration;
        #[rust_name = "qlist_default_QSslConfiguration"]
        fn construct() -> QList_QSslConfiguration;
        #[rust_name = "qlist_drop_QSslConfiguration"]
        fn drop(_: &mut QList_QSslConfiguration);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QSslConfiguration"]
        fn qlistReserve(_: &mut QList_QSslConfiguration, size: isize);
        #[rust_name = "qlist_append_QSslConfiguration"]
        fn qlistAppend(_: &mut QList_QSslConfiguration, _: &QSslConfiguration);
        #[rust_name = "qlist_get_unchecked_QSslConfiguration"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QSslConfiguration,
            pos: isize,
        ) -> &QSslConfiguration;
        #[rust_name = "qlist_index_of_QSslConfiguration"]
        fn qlistIndexOf(_: &QList_QSslConfiguration, _: &QSslConfiguration) -> isize;
        #[rust_name = "qlist_insert_QSslConfiguration"]
        fn qlistInsert(_: &mut QList_QSslConfiguration, _: isize, _: &QSslConfiguration);
        #[rust_name = "qlist_remove_QSslConfiguration"]
        fn qlistRemove(_: &mut QList_QSslConfiguration, _: isize);
        #[rust_name = "qlist_len_QSslConfiguration"]
        fn qlistLen(_: &QList_QSslConfiguration) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslConfiguration) {
    ffi::qlist_clear_QSslConfiguration(v);
}

pub(crate) fn contains(v: &ffi::QList_QSslConfiguration, item: &ffi::QSslConfiguration) -> bool {
    ffi::qlist_contains_QSslConfiguration(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslConfiguration, size: isize) {
    ffi::qlist_reserve_QSslConfiguration(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QSslConfiguration, value: &ffi::QSslConfiguration) {
    ffi::qlist_append_QSslConfiguration(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QSslConfiguration) -> ffi::QList_QSslConfiguration {
    ffi::qlist_clone_QSslConfiguration(s)
}

pub(crate) fn default() -> ffi::QList_QSslConfiguration {
    ffi::qlist_default_QSslConfiguration()
}

pub(crate) fn drop(s: &mut ffi::QList_QSslConfiguration) {
    ffi::qlist_drop_QSslConfiguration(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QSslConfiguration,
    pos: isize,
) -> &ffi::QSslConfiguration {
    ffi::qlist_get_unchecked_QSslConfiguration(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QSslConfiguration, value: &ffi::QSslConfiguration) -> isize {
    ffi::qlist_index_of_QSslConfiguration(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QSslConfiguration,
    pos: isize,
    value: &ffi::QSslConfiguration,
) {
    ffi::qlist_insert_QSslConfiguration(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslConfiguration) -> isize {
    ffi::qlist_len_QSslConfiguration(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslConfiguration, pos: isize) {
    ffi::qlist_remove_QSslConfiguration(s, pos);
}
