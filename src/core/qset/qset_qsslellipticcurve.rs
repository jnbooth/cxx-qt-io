//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qset/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-io/qsslellipticcurve.h");
        type QSslEllipticCurve = crate::QSslEllipticCurve;

        include!("cxx-qt-io/qset_qsslellipticcurve.h");
        type QSet_QSslEllipticCurve = cxx_qt_lib::QSet<QSslEllipticCurve>;
    }

    #[namespace = "rust::cxxqtio1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_clear_QSslEllipticCurve"]
        fn qsetClear(set: &mut QSet_QSslEllipticCurve);
        #[rust_name = "qset_contains_QSslEllipticCurve"]
        fn qsetContains(set: &QSet_QSslEllipticCurve, _: &QSslEllipticCurve) -> bool;
        #[rust_name = "qset_remove_QSslEllipticCurve"]
        fn qsetRemove(set: &mut QSet_QSslEllipticCurve, _: &QSslEllipticCurve) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QSslEllipticCurve"]
        fn construct(_: &QSet_QSslEllipticCurve) -> QSet_QSslEllipticCurve;
        #[rust_name = "qset_default_QSslEllipticCurve"]
        fn construct() -> QSet_QSslEllipticCurve;
        #[rust_name = "qset_drop_QSslEllipticCurve"]
        fn drop(_: &mut QSet_QSslEllipticCurve);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_get_unchecked_QSslEllipticCurve"]
        unsafe fn qsetGetUnchecked(set: &QSet_QSslEllipticCurve, pos: isize) -> &QSslEllipticCurve;
        #[rust_name = "qset_insert_QSslEllipticCurve"]
        fn qsetInsert(_: &mut QSet_QSslEllipticCurve, _: &QSslEllipticCurve);
        #[rust_name = "qset_len_QSslEllipticCurve"]
        fn qsetLen(_: &QSet_QSslEllipticCurve) -> isize;
        #[rust_name = "qset_reserve_QSslEllipticCurve"]
        fn qsetReserve(_: &mut QSet_QSslEllipticCurve, size: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QSet_QSslEllipticCurve) {
    ffi::qset_clear_QSslEllipticCurve(v);
}

pub(crate) fn contains(v: &ffi::QSet_QSslEllipticCurve, item: &ffi::QSslEllipticCurve) -> bool {
    ffi::qset_contains_QSslEllipticCurve(v, item)
}

pub(crate) fn remove(v: &mut ffi::QSet_QSslEllipticCurve, item: &ffi::QSslEllipticCurve) -> bool {
    ffi::qset_remove_QSslEllipticCurve(v, item)
}

pub(crate) fn clone(s: &ffi::QSet_QSslEllipticCurve) -> ffi::QSet_QSslEllipticCurve {
    ffi::qset_clone_QSslEllipticCurve(s)
}

pub(crate) fn default() -> ffi::QSet_QSslEllipticCurve {
    ffi::qset_default_QSslEllipticCurve()
}

pub(crate) fn drop(s: &mut ffi::QSet_QSslEllipticCurve) {
    ffi::qset_drop_QSslEllipticCurve(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QSet_QSslEllipticCurve,
    pos: isize,
) -> &ffi::QSslEllipticCurve {
    ffi::qset_get_unchecked_QSslEllipticCurve(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_QSslEllipticCurve, value: &ffi::QSslEllipticCurve) {
    ffi::qset_insert_QSslEllipticCurve(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QSslEllipticCurve) -> isize {
    ffi::qset_len_QSslEllipticCurve(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_QSslEllipticCurve, size: isize) {
    ffi::qset_reserve_QSslEllipticCurve(s, size);
}
