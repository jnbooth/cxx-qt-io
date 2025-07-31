//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

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
        #[rust_name = "qlist_clear_QSslEllipticCurve"]
        fn qlistClear(list: &mut QList_QSslEllipticCurve);
        #[rust_name = "qlist_contains_QSslEllipticCurve"]
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
        #[rust_name = "qlist_reserve_QSslEllipticCurve"]
        fn qlistReserve(_: &mut QList_QSslEllipticCurve, size: isize);
        #[rust_name = "qlist_append_QSslEllipticCurve"]
        fn qlistAppend(_: &mut QList_QSslEllipticCurve, _: &QSslEllipticCurve);
        #[rust_name = "qlist_get_unchecked_QSslEllipticCurve"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QSslEllipticCurve,
            pos: isize,
        ) -> &QSslEllipticCurve;
        #[rust_name = "qlist_index_of_QSslEllipticCurve"]
        fn qlistIndexOf(_: &QList_QSslEllipticCurve, _: &QSslEllipticCurve) -> isize;
        #[rust_name = "qlist_insert_QSslEllipticCurve"]
        fn qlistInsert(_: &mut QList_QSslEllipticCurve, _: isize, _: &QSslEllipticCurve);
        #[rust_name = "qlist_remove_QSslEllipticCurve"]
        fn qlistRemove(_: &mut QList_QSslEllipticCurve, _: isize);
        #[rust_name = "qlist_len_QSslEllipticCurve"]
        fn qlistLen(_: &QList_QSslEllipticCurve) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslEllipticCurve) {
    ffi::qlist_clear_QSslEllipticCurve(v);
}

pub(crate) fn contains(v: &ffi::QList_QSslEllipticCurve, item: &ffi::QSslEllipticCurve) -> bool {
    ffi::qlist_contains_QSslEllipticCurve(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslEllipticCurve, size: isize) {
    ffi::qlist_reserve_QSslEllipticCurve(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QSslEllipticCurve, value: &ffi::QSslEllipticCurve) {
    ffi::qlist_append_QSslEllipticCurve(v, value);
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
    unsafe { ffi::qlist_get_unchecked_QSslEllipticCurve(s, pos) }
}

pub(crate) fn index_of(v: &ffi::QList_QSslEllipticCurve, value: &ffi::QSslEllipticCurve) -> isize {
    ffi::qlist_index_of_QSslEllipticCurve(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QSslEllipticCurve,
    pos: isize,
    value: &ffi::QSslEllipticCurve,
) {
    ffi::qlist_insert_QSslEllipticCurve(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslEllipticCurve) -> isize {
    ffi::qlist_len_QSslEllipticCurve(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslEllipticCurve, pos: isize) {
    ffi::qlist_remove_QSslEllipticCurve(s, pos);
}
