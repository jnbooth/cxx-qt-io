//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qsslellipticcurve.h");
        type QSslEllipticCurve = crate::QSslEllipticCurve;

        include!("cxx-qt-io/qlist_qsslellipticcurve.h");
        type QList_QSslEllipticCurve = cxx_qt_lib::QList<QSslEllipticCurve>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_QSslEllipticCurve"]
        fn qlistClear(list: &mut QList_QSslEllipticCurve);
        #[rust_name = "cxx_contains"]
        fn qlistContains(list: &QList_QSslEllipticCurve, _: &QSslEllipticCurve) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSslEllipticCurve"]
        fn construct(_: &QList_QSslEllipticCurve) -> QList_QSslEllipticCurve;
        #[rust_name = "qlist_default_QSslEllipticCurve"]
        fn construct() -> QList_QSslEllipticCurve;
        #[rust_name = "qlist_drop_QSslEllipticCurve"]
        fn drop(_: &mut QList_QSslEllipticCurve);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSslEllipticCurve"]
        fn qlistReserve(_: &mut QList_QSslEllipticCurve, size: isize);
        #[rust_name = "append_QSslEllipticCurve"]
        fn qlistAppend(_: &mut QList_QSslEllipticCurve, _: &QSslEllipticCurve);
        #[rust_name = "get_unchecked_QSslEllipticCurve"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QSslEllipticCurve,
            pos: isize,
        ) -> &QSslEllipticCurve;
        #[rust_name = "index_of_QSslEllipticCurve"]
        fn qlistIndexOf(_: &QList_QSslEllipticCurve, _: &QSslEllipticCurve) -> isize;
        #[rust_name = "insert_QSslEllipticCurve"]
        fn qlistInsert(_: &mut QList_QSslEllipticCurve, _: isize, _: &QSslEllipticCurve);
        #[rust_name = "remove_QSslEllipticCurve"]
        fn qlistRemove(_: &mut QList_QSslEllipticCurve, _: isize);
        #[rust_name = "len_QSslEllipticCurve"]
        fn qlistLen(_: &QList_QSslEllipticCurve) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslEllipticCurve) {
    ffi::cxx_clear_qlist_QSslEllipticCurve(v);
}

pub(crate) fn contains(v: &ffi::QList_QSslEllipticCurve, item: &ffi::QSslEllipticCurve) -> bool {
    ffi::cxx_contains(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslEllipticCurve, size: isize) {
    ffi::reserve_QSslEllipticCurve(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QSslEllipticCurve, value: &ffi::QSslEllipticCurve) {
    ffi::append_QSslEllipticCurve(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QSslEllipticCurve) -> ffi::QList_QSslEllipticCurve {
    ffi::qlist_clone_QSslEllipticCurve(s)
}

pub(crate) fn default() -> ffi::QList_QSslEllipticCurve {
    ffi::qlist_default_QSslEllipticCurve()
}

pub(crate) fn drop(s: &mut ffi::QList_QSslEllipticCurve) {
    ffi::qlist_drop_QSslEllipticCurve(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QSslEllipticCurve,
    pos: isize,
) -> &ffi::QSslEllipticCurve {
    ffi::get_unchecked_QSslEllipticCurve(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QSslEllipticCurve, value: &ffi::QSslEllipticCurve) -> isize {
    ffi::index_of_QSslEllipticCurve(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QSslEllipticCurve,
    pos: isize,
    value: &ffi::QSslEllipticCurve,
) {
    ffi::insert_QSslEllipticCurve(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslEllipticCurve) -> isize {
    ffi::len_QSslEllipticCurve(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslEllipticCurve, pos: isize) {
    ffi::remove_QSslEllipticCurve(s, pos);
}
