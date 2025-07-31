//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qnetworkcookie.h");
        type QNetworkCookie = crate::QNetworkCookie;

        include!("cxx-qt-io/qlist_qnetworkcookie.h");
        type QList_QNetworkCookie = cxx_qt_lib::QList<QNetworkCookie>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QNetworkCookie"]
        fn qlistClear(list: &mut QList_QNetworkCookie);
        #[rust_name = "qlist_contains_QNetworkCookie"]
        fn qlistContains(list: &QList_QNetworkCookie, _: &QNetworkCookie) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QNetworkCookie"]
        fn construct(_: &QList_QNetworkCookie) -> QList_QNetworkCookie;
        #[rust_name = "qlist_default_QNetworkCookie"]
        fn construct() -> QList_QNetworkCookie;
        #[rust_name = "qlist_drop_QNetworkCookie"]
        fn drop(_: &mut QList_QNetworkCookie);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QNetworkCookie"]
        fn qlistReserve(_: &mut QList_QNetworkCookie, size: isize);
        #[rust_name = "qlist_append_QNetworkCookie"]
        fn qlistAppend(_: &mut QList_QNetworkCookie, _: &QNetworkCookie);
        #[rust_name = "qlist_get_unchecked_QNetworkCookie"]
        unsafe fn qlistGetUnchecked(set: &QList_QNetworkCookie, pos: isize) -> &QNetworkCookie;
        #[rust_name = "qlist_index_of_QNetworkCookie"]
        fn qlistIndexOf(_: &QList_QNetworkCookie, _: &QNetworkCookie) -> isize;
        #[rust_name = "qlist_insert_QNetworkCookie"]
        fn qlistInsert(_: &mut QList_QNetworkCookie, _: isize, _: &QNetworkCookie);
        #[rust_name = "qlist_remove_QNetworkCookie"]
        fn qlistRemove(_: &mut QList_QNetworkCookie, _: isize);
        #[rust_name = "qlist_len_QNetworkCookie"]
        fn qlistLen(_: &QList_QNetworkCookie) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QNetworkCookie) {
    ffi::qlist_clear_QNetworkCookie(v);
}

pub(crate) fn contains(v: &ffi::QList_QNetworkCookie, item: &ffi::QNetworkCookie) -> bool {
    ffi::qlist_contains_QNetworkCookie(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QNetworkCookie, size: isize) {
    ffi::qlist_reserve_QNetworkCookie(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QNetworkCookie, value: &ffi::QNetworkCookie) {
    ffi::qlist_append_QNetworkCookie(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QNetworkCookie) -> ffi::QList_QNetworkCookie {
    ffi::qlist_clone_QNetworkCookie(s)
}

pub(crate) fn default() -> ffi::QList_QNetworkCookie {
    ffi::qlist_default_QNetworkCookie()
}

pub(crate) fn drop(s: &mut ffi::QList_QNetworkCookie) {
    ffi::qlist_drop_QNetworkCookie(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QNetworkCookie,
    pos: isize,
) -> &ffi::QNetworkCookie {
    ffi::qlist_get_unchecked_QNetworkCookie(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QNetworkCookie, value: &ffi::QNetworkCookie) -> isize {
    ffi::qlist_index_of_QNetworkCookie(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QNetworkCookie, pos: isize, value: &ffi::QNetworkCookie) {
    ffi::qlist_insert_QNetworkCookie(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QNetworkCookie) -> isize {
    ffi::qlist_len_QNetworkCookie(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QNetworkCookie, pos: isize) {
    ffi::qlist_remove_QNetworkCookie(s, pos);
}
