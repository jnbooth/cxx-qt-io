//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qsslpresharedkeyauthenticator.h");
        type QSslPreSharedKeyAuthenticator = crate::QSslPreSharedKeyAuthenticator;

        include!("cxx-qt-io/qlist_qsslpresharedkeyauthenticator.h");
        type QList_QSslPreSharedKeyAuthenticator = cxx_qt_lib::QList<QSslPreSharedKeyAuthenticator>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qlist_clear_QSslPreSharedKeyAuthenticator"]
        fn qlistClear(list: &mut QList_QSslPreSharedKeyAuthenticator);
        #[rust_name = "cxx_qlist_contains_QSslPreSharedKeyAuthenticator"]
        fn qlistContains(
            list: &QList_QSslPreSharedKeyAuthenticator,
            _: &QSslPreSharedKeyAuthenticator,
        ) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSslPreSharedKeyAuthenticator"]
        fn construct(
            _: &QList_QSslPreSharedKeyAuthenticator,
        ) -> QList_QSslPreSharedKeyAuthenticator;
        #[rust_name = "qlist_default_QSslPreSharedKeyAuthenticator"]
        fn construct() -> QList_QSslPreSharedKeyAuthenticator;
        #[rust_name = "qlist_drop_QSslPreSharedKeyAuthenticator"]
        fn drop(_: &mut QList_QSslPreSharedKeyAuthenticator);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSslPreSharedKeyAuthenticator"]
        fn qlistReserve(_: &mut QList_QSslPreSharedKeyAuthenticator, size: isize);
        #[rust_name = "append_QSslPreSharedKeyAuthenticator"]
        fn qlistAppend(
            _: &mut QList_QSslPreSharedKeyAuthenticator,
            _: &QSslPreSharedKeyAuthenticator,
        );
        #[rust_name = "get_unchecked_QSslPreSharedKeyAuthenticator"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QSslPreSharedKeyAuthenticator,
            pos: isize,
        ) -> &QSslPreSharedKeyAuthenticator;
        #[rust_name = "index_of_QSslPreSharedKeyAuthenticator"]
        fn qlistIndexOf(
            _: &QList_QSslPreSharedKeyAuthenticator,
            _: &QSslPreSharedKeyAuthenticator,
        ) -> isize;
        #[rust_name = "insert_QSslPreSharedKeyAuthenticator"]
        fn qlistInsert(
            _: &mut QList_QSslPreSharedKeyAuthenticator,
            _: isize,
            _: &QSslPreSharedKeyAuthenticator,
        );
        #[rust_name = "remove_QSslPreSharedKeyAuthenticator"]
        fn qlistRemove(_: &mut QList_QSslPreSharedKeyAuthenticator, _: isize);
        #[rust_name = "len_QSslPreSharedKeyAuthenticator"]
        fn qlistLen(_: &QList_QSslPreSharedKeyAuthenticator) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslPreSharedKeyAuthenticator) {
    ffi::cxx_qlist_clear_QSslPreSharedKeyAuthenticator(v);
}

pub(crate) fn contains(
    v: &ffi::QList_QSslPreSharedKeyAuthenticator,
    item: &ffi::QSslPreSharedKeyAuthenticator,
) -> bool {
    ffi::cxx_qlist_contains_QSslPreSharedKeyAuthenticator(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslPreSharedKeyAuthenticator, size: isize) {
    ffi::reserve_QSslPreSharedKeyAuthenticator(v, size);
}

pub(crate) fn append(
    v: &mut ffi::QList_QSslPreSharedKeyAuthenticator,
    value: &ffi::QSslPreSharedKeyAuthenticator,
) {
    ffi::append_QSslPreSharedKeyAuthenticator(v, value);
}

pub(crate) fn clone(
    s: &ffi::QList_QSslPreSharedKeyAuthenticator,
) -> ffi::QList_QSslPreSharedKeyAuthenticator {
    ffi::qlist_clone_QSslPreSharedKeyAuthenticator(s)
}

pub(crate) fn default() -> ffi::QList_QSslPreSharedKeyAuthenticator {
    ffi::qlist_default_QSslPreSharedKeyAuthenticator()
}

pub(crate) fn drop(s: &mut ffi::QList_QSslPreSharedKeyAuthenticator) {
    ffi::qlist_drop_QSslPreSharedKeyAuthenticator(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QSslPreSharedKeyAuthenticator,
    pos: isize,
) -> &ffi::QSslPreSharedKeyAuthenticator {
    ffi::get_unchecked_QSslPreSharedKeyAuthenticator(s, pos)
}

pub(crate) fn index_of(
    v: &ffi::QList_QSslPreSharedKeyAuthenticator,
    value: &ffi::QSslPreSharedKeyAuthenticator,
) -> isize {
    ffi::index_of_QSslPreSharedKeyAuthenticator(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QSslPreSharedKeyAuthenticator,
    pos: isize,
    value: &ffi::QSslPreSharedKeyAuthenticator,
) {
    ffi::insert_QSslPreSharedKeyAuthenticator(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslPreSharedKeyAuthenticator) -> isize {
    ffi::len_QSslPreSharedKeyAuthenticator(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslPreSharedKeyAuthenticator, pos: isize) {
    ffi::remove_QSslPreSharedKeyAuthenticator(s, pos);
}
