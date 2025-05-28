//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qhstspolicy.h");
        type QHstsPolicy = crate::QHstsPolicy;

        include!("cxx-qt-io/qlist_qhstspolicy.h");
        type QList_QHstsPolicy = cxx_qt_lib::QList<QHstsPolicy>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qlist_clear_QHstsPolicy"]
        fn qlistClear(list: &mut QList_QHstsPolicy);
        #[rust_name = "cxx_qlist_contains_QHstsPolicy"]
        fn qlistContains(list: &QList_QHstsPolicy, _: &QHstsPolicy) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QHstsPolicy"]
        fn construct(_: &QList_QHstsPolicy) -> QList_QHstsPolicy;
        #[rust_name = "qlist_default_QHstsPolicy"]
        fn construct() -> QList_QHstsPolicy;
        #[rust_name = "qlist_drop_QHstsPolicy"]
        fn drop(_: &mut QList_QHstsPolicy);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QHstsPolicy"]
        fn qlistReserve(_: &mut QList_QHstsPolicy, size: isize);
        #[rust_name = "append_QHstsPolicy"]
        fn qlistAppend(_: &mut QList_QHstsPolicy, _: &QHstsPolicy);
        #[rust_name = "get_unchecked_QHstsPolicy"]
        unsafe fn qlistGetUnchecked(set: &QList_QHstsPolicy, pos: isize) -> &QHstsPolicy;
        #[rust_name = "index_of_QHstsPolicy"]
        fn qlistIndexOf(_: &QList_QHstsPolicy, _: &QHstsPolicy) -> isize;
        #[rust_name = "insert_QHstsPolicy"]
        fn qlistInsert(_: &mut QList_QHstsPolicy, _: isize, _: &QHstsPolicy);
        #[rust_name = "remove_QHstsPolicy"]
        fn qlistRemove(_: &mut QList_QHstsPolicy, _: isize);
        #[rust_name = "len_QHstsPolicy"]
        fn qlistLen(_: &QList_QHstsPolicy) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QHstsPolicy) {
    ffi::cxx_qlist_clear_QHstsPolicy(v);
}

pub(crate) fn contains(v: &ffi::QList_QHstsPolicy, item: &ffi::QHstsPolicy) -> bool {
    ffi::cxx_qlist_contains_QHstsPolicy(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QHstsPolicy, size: isize) {
    ffi::reserve_QHstsPolicy(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QHstsPolicy, value: &ffi::QHstsPolicy) {
    ffi::append_QHstsPolicy(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QHstsPolicy) -> ffi::QList_QHstsPolicy {
    ffi::qlist_clone_QHstsPolicy(s)
}

pub(crate) fn default() -> ffi::QList_QHstsPolicy {
    ffi::qlist_default_QHstsPolicy()
}

pub(crate) fn drop(s: &mut ffi::QList_QHstsPolicy) {
    ffi::qlist_drop_QHstsPolicy(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QHstsPolicy, pos: isize) -> &ffi::QHstsPolicy {
    ffi::get_unchecked_QHstsPolicy(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QHstsPolicy, value: &ffi::QHstsPolicy) -> isize {
    ffi::index_of_QHstsPolicy(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QHstsPolicy, pos: isize, value: &ffi::QHstsPolicy) {
    ffi::insert_QHstsPolicy(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QHstsPolicy) -> isize {
    ffi::len_QHstsPolicy(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QHstsPolicy, pos: isize) {
    ffi::remove_QHstsPolicy(s, pos);
}
