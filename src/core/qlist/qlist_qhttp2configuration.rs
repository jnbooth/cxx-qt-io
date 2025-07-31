//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qhttp2configuration.h");
        type QHttp2Configuration = crate::QHttp2Configuration;

        include!("cxx-qt-io/qlist_qhttp2configuration.h");
        type QList_QHttp2Configuration = cxx_qt_lib::QList<QHttp2Configuration>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QHttp2Configuration"]
        fn qlistClear(list: &mut QList_QHttp2Configuration);
        #[rust_name = "qlist_contains_QHttp2Configuration"]
        fn qlistContains(list: &QList_QHttp2Configuration, _: &QHttp2Configuration) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QHttp2Configuration"]
        fn construct(_: &QList_QHttp2Configuration) -> QList_QHttp2Configuration;
        #[rust_name = "qlist_default_QHttp2Configuration"]
        fn construct() -> QList_QHttp2Configuration;
        #[rust_name = "qlist_drop_QHttp2Configuration"]
        fn drop(_: &mut QList_QHttp2Configuration);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QHttp2Configuration"]
        fn qlistReserve(_: &mut QList_QHttp2Configuration, size: isize);
        #[rust_name = "qlist_append_QHttp2Configuration"]
        fn qlistAppend(_: &mut QList_QHttp2Configuration, _: &QHttp2Configuration);
        #[rust_name = "qlist_get_unchecked_QHttp2Configuration"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QHttp2Configuration,
            pos: isize,
        ) -> &QHttp2Configuration;
        #[rust_name = "qlist_index_of_QHttp2Configuration"]
        fn qlistIndexOf(_: &QList_QHttp2Configuration, _: &QHttp2Configuration) -> isize;
        #[rust_name = "qlist_insert_QHttp2Configuration"]
        fn qlistInsert(_: &mut QList_QHttp2Configuration, _: isize, _: &QHttp2Configuration);
        #[rust_name = "qlist_remove_QHttp2Configuration"]
        fn qlistRemove(_: &mut QList_QHttp2Configuration, _: isize);
        #[rust_name = "qlist_len_QHttp2Configuration"]
        fn qlistLen(_: &QList_QHttp2Configuration) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QHttp2Configuration) {
    ffi::qlist_clear_QHttp2Configuration(v);
}

pub(crate) fn contains(
    v: &ffi::QList_QHttp2Configuration,
    item: &ffi::QHttp2Configuration,
) -> bool {
    ffi::qlist_contains_QHttp2Configuration(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QHttp2Configuration, size: isize) {
    ffi::qlist_reserve_QHttp2Configuration(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QHttp2Configuration, value: &ffi::QHttp2Configuration) {
    ffi::qlist_append_QHttp2Configuration(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QHttp2Configuration) -> ffi::QList_QHttp2Configuration {
    ffi::qlist_clone_QHttp2Configuration(s)
}

pub(crate) fn default() -> ffi::QList_QHttp2Configuration {
    ffi::qlist_default_QHttp2Configuration()
}

pub(crate) fn drop(s: &mut ffi::QList_QHttp2Configuration) {
    ffi::qlist_drop_QHttp2Configuration(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QHttp2Configuration,
    pos: isize,
) -> &ffi::QHttp2Configuration {
    ffi::qlist_get_unchecked_QHttp2Configuration(s, pos)
}

pub(crate) fn index_of(
    v: &ffi::QList_QHttp2Configuration,
    value: &ffi::QHttp2Configuration,
) -> isize {
    ffi::qlist_index_of_QHttp2Configuration(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QHttp2Configuration,
    pos: isize,
    value: &ffi::QHttp2Configuration,
) {
    ffi::qlist_insert_QHttp2Configuration(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QHttp2Configuration) -> isize {
    ffi::qlist_len_QHttp2Configuration(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QHttp2Configuration, pos: isize) {
    ffi::qlist_remove_QHttp2Configuration(s, pos);
}
