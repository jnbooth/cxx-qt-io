//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qsslcipher.h");
        type QSslCipher = crate::QSslCipher;

        include!("cxx-qt-io/qlist_qsslcipher.h");
        type QList_QSslCipher = cxx_qt_lib::QList<QSslCipher>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_QSslCipher"]
        fn qlistClear(list: &mut QList_QSslCipher);
        #[rust_name = "cxx_contains"]
        fn qlistContains(list: &QList_QSslCipher, _: &QSslCipher) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSslCipher"]
        fn construct(_: &QList_QSslCipher) -> QList_QSslCipher;
        #[rust_name = "qlist_default_QSslCipher"]
        fn construct() -> QList_QSslCipher;
        #[rust_name = "qlist_drop_QSslCipher"]
        fn drop(_: &mut QList_QSslCipher);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSslCipher"]
        fn qlistReserve(_: &mut QList_QSslCipher, size: isize);
        #[rust_name = "append_QSslCipher"]
        fn qlistAppend(_: &mut QList_QSslCipher, _: &QSslCipher);
        #[rust_name = "get_unchecked_QSslCipher"]
        unsafe fn qlistGetUnchecked(set: &QList_QSslCipher, pos: isize) -> &QSslCipher;
        #[rust_name = "index_of_QSslCipher"]
        fn qlistIndexOf(_: &QList_QSslCipher, _: &QSslCipher) -> isize;
        #[rust_name = "insert_QSslCipher"]
        fn qlistInsert(_: &mut QList_QSslCipher, _: isize, _: &QSslCipher);
        #[rust_name = "remove_QSslCipher"]
        fn qlistRemove(_: &mut QList_QSslCipher, _: isize);
        #[rust_name = "len_QSslCipher"]
        fn qlistLen(_: &QList_QSslCipher) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslCipher) {
    ffi::cxx_clear_qlist_QSslCipher(v);
}

pub(crate) fn contains(v: &ffi::QList_QSslCipher, item: &ffi::QSslCipher) -> bool {
    ffi::cxx_contains(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslCipher, size: isize) {
    ffi::reserve_QSslCipher(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QSslCipher, value: &ffi::QSslCipher) {
    ffi::append_QSslCipher(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QSslCipher) -> ffi::QList_QSslCipher {
    ffi::qlist_clone_QSslCipher(s)
}

pub(crate) fn default() -> ffi::QList_QSslCipher {
    ffi::qlist_default_QSslCipher()
}

pub(crate) fn drop(s: &mut ffi::QList_QSslCipher) {
    ffi::qlist_drop_QSslCipher(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QSslCipher, pos: isize) -> &ffi::QSslCipher {
    ffi::get_unchecked_QSslCipher(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QSslCipher, value: &ffi::QSslCipher) -> isize {
    ffi::index_of_QSslCipher(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QSslCipher, pos: isize, value: &ffi::QSslCipher) {
    ffi::insert_QSslCipher(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslCipher) -> isize {
    ffi::len_QSslCipher(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslCipher, pos: isize) {
    ffi::remove_QSslCipher(s, pos);
}
