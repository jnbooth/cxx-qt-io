//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qdeadlinetimer.h");
        type QDeadlineTimer = crate::QDeadlineTimer;

        include!("cxx-qt-io/qlist_qdeadlinetimer.h");
        type QList_QDeadlineTimer = cxx_qt_lib::QList<QDeadlineTimer>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QDeadlineTimer"]
        fn qlistClear(list: &mut QList_QDeadlineTimer);
        #[rust_name = "qlist_contains_QDeadlineTimer"]
        fn qlistContains(list: &QList_QDeadlineTimer, _: &QDeadlineTimer) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QDeadlineTimer"]
        fn construct(_: &QList_QDeadlineTimer) -> QList_QDeadlineTimer;
        #[rust_name = "qlist_default_QDeadlineTimer"]
        fn construct() -> QList_QDeadlineTimer;
        #[rust_name = "qlist_drop_QDeadlineTimer"]
        fn drop(_: &mut QList_QDeadlineTimer);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QDeadlineTimer"]
        fn qlistReserve(_: &mut QList_QDeadlineTimer, size: isize);
        #[rust_name = "qlist_append_QDeadlineTimer"]
        fn qlistAppend(_: &mut QList_QDeadlineTimer, _: &QDeadlineTimer);
        #[rust_name = "qlist_get_unchecked_QDeadlineTimer"]
        unsafe fn qlistGetUnchecked(set: &QList_QDeadlineTimer, pos: isize) -> &QDeadlineTimer;
        #[rust_name = "qlist_index_of_QDeadlineTimer"]
        fn qlistIndexOf(_: &QList_QDeadlineTimer, _: &QDeadlineTimer) -> isize;
        #[rust_name = "qlist_insert_QDeadlineTimer"]
        fn qlistInsert(_: &mut QList_QDeadlineTimer, _: isize, _: &QDeadlineTimer);
        #[rust_name = "qlist_remove_QDeadlineTimer"]
        fn qlistRemove(_: &mut QList_QDeadlineTimer, _: isize);
        #[rust_name = "qlist_len_QDeadlineTimer"]
        fn qlistLen(_: &QList_QDeadlineTimer) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QDeadlineTimer) {
    ffi::qlist_clear_QDeadlineTimer(v);
}

pub(crate) fn contains(v: &ffi::QList_QDeadlineTimer, item: &ffi::QDeadlineTimer) -> bool {
    ffi::qlist_contains_QDeadlineTimer(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QDeadlineTimer, size: isize) {
    ffi::qlist_reserve_QDeadlineTimer(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QDeadlineTimer, value: &ffi::QDeadlineTimer) {
    ffi::qlist_append_QDeadlineTimer(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QDeadlineTimer) -> ffi::QList_QDeadlineTimer {
    ffi::qlist_clone_QDeadlineTimer(s)
}

pub(crate) fn default() -> ffi::QList_QDeadlineTimer {
    ffi::qlist_default_QDeadlineTimer()
}

pub(crate) fn drop(s: &mut ffi::QList_QDeadlineTimer) {
    ffi::qlist_drop_QDeadlineTimer(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QDeadlineTimer,
    pos: isize,
) -> &ffi::QDeadlineTimer {
    ffi::qlist_get_unchecked_QDeadlineTimer(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QDeadlineTimer, value: &ffi::QDeadlineTimer) -> isize {
    ffi::qlist_index_of_QDeadlineTimer(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QDeadlineTimer, pos: isize, value: &ffi::QDeadlineTimer) {
    ffi::qlist_insert_QDeadlineTimer(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QDeadlineTimer) -> isize {
    ffi::qlist_len_QDeadlineTimer(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QDeadlineTimer, pos: isize) {
    ffi::qlist_remove_QDeadlineTimer(s, pos);
}
