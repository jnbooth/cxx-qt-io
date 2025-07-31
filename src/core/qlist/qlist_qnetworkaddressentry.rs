//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qnetworkaddressentry.h");
        type QNetworkAddressEntry = crate::QNetworkAddressEntry;

        include!("cxx-qt-io/qlist_qnetworkaddressentry.h");
        type QList_QNetworkAddressEntry = cxx_qt_lib::QList<QNetworkAddressEntry>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QNetworkAddressEntry"]
        fn qlistClear(list: &mut QList_QNetworkAddressEntry);
        #[rust_name = "qlist_contains_QNetworkAddressEntry"]
        fn qlistContains(list: &QList_QNetworkAddressEntry, _: &QNetworkAddressEntry) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QNetworkAddressEntry"]
        fn construct(_: &QList_QNetworkAddressEntry) -> QList_QNetworkAddressEntry;
        #[rust_name = "qlist_default_QNetworkAddressEntry"]
        fn construct() -> QList_QNetworkAddressEntry;
        #[rust_name = "qlist_drop_QNetworkAddressEntry"]
        fn drop(_: &mut QList_QNetworkAddressEntry);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QNetworkAddressEntry"]
        fn qlistReserve(_: &mut QList_QNetworkAddressEntry, size: isize);
        #[rust_name = "qlist_append_QNetworkAddressEntry"]
        fn qlistAppend(_: &mut QList_QNetworkAddressEntry, _: &QNetworkAddressEntry);
        #[rust_name = "qlist_get_unchecked_QNetworkAddressEntry"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QNetworkAddressEntry,
            pos: isize,
        ) -> &QNetworkAddressEntry;
        #[rust_name = "qlist_index_of_QNetworkAddressEntry"]
        fn qlistIndexOf(_: &QList_QNetworkAddressEntry, _: &QNetworkAddressEntry) -> isize;
        #[rust_name = "qlist_insert_QNetworkAddressEntry"]
        fn qlistInsert(_: &mut QList_QNetworkAddressEntry, _: isize, _: &QNetworkAddressEntry);
        #[rust_name = "qlist_remove_QNetworkAddressEntry"]
        fn qlistRemove(_: &mut QList_QNetworkAddressEntry, _: isize);
        #[rust_name = "qlist_len_QNetworkAddressEntry"]
        fn qlistLen(_: &QList_QNetworkAddressEntry) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QNetworkAddressEntry) {
    ffi::qlist_clear_QNetworkAddressEntry(v);
}

pub(crate) fn contains(
    v: &ffi::QList_QNetworkAddressEntry,
    item: &ffi::QNetworkAddressEntry,
) -> bool {
    ffi::qlist_contains_QNetworkAddressEntry(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QNetworkAddressEntry, size: isize) {
    ffi::qlist_reserve_QNetworkAddressEntry(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QNetworkAddressEntry, value: &ffi::QNetworkAddressEntry) {
    ffi::qlist_append_QNetworkAddressEntry(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QNetworkAddressEntry) -> ffi::QList_QNetworkAddressEntry {
    ffi::qlist_clone_QNetworkAddressEntry(s)
}

pub(crate) fn default() -> ffi::QList_QNetworkAddressEntry {
    ffi::qlist_default_QNetworkAddressEntry()
}

pub(crate) fn drop(s: &mut ffi::QList_QNetworkAddressEntry) {
    ffi::qlist_drop_QNetworkAddressEntry(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QNetworkAddressEntry,
    pos: isize,
) -> &ffi::QNetworkAddressEntry {
    unsafe { ffi::qlist_get_unchecked_QNetworkAddressEntry(s, pos) }
}

pub(crate) fn index_of(
    v: &ffi::QList_QNetworkAddressEntry,
    value: &ffi::QNetworkAddressEntry,
) -> isize {
    ffi::qlist_index_of_QNetworkAddressEntry(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QNetworkAddressEntry,
    pos: isize,
    value: &ffi::QNetworkAddressEntry,
) {
    ffi::qlist_insert_QNetworkAddressEntry(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QNetworkAddressEntry) -> isize {
    ffi::qlist_len_QNetworkAddressEntry(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QNetworkAddressEntry, pos: isize) {
    ffi::qlist_remove_QNetworkAddressEntry(s, pos);
}
