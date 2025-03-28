use crate::QHttpHeadersEntry;

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qlist_qhttpheadersentry.h");
        type QHttpHeadersEntry = crate::QHttpHeadersEntry;
        type QList_QHttpHeadersEntry = cxx_qt_lib::QList<QHttpHeadersEntry>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_qhttpheadersentry"]
        fn qlistClear(list: &mut QList_QHttpHeadersEntry);
        #[rust_name = "cxx_contains"]
        fn qlistContains(list: &QList_QHttpHeadersEntry, _: &QHttpHeadersEntry) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QHttpHeadersEntry"]
        fn construct(_: &QList_QHttpHeadersEntry) -> QList_QHttpHeadersEntry;
        #[rust_name = "qlist_default_QHttpHeadersEntry"]
        fn construct() -> QList_QHttpHeadersEntry;
        #[rust_name = "qlist_drop_QHttpHeadersEntry"]
        fn drop(_: &mut QList_QHttpHeadersEntry);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QHttpHeadersEntry"]
        fn qlistReserve(_: &mut QList_QHttpHeadersEntry, size: isize);
        #[rust_name = "append_QHttpHeadersEntry"]
        fn qlistAppend(_: &mut QList_QHttpHeadersEntry, _: &QHttpHeadersEntry);
        #[rust_name = "get_unchecked_QHttpHeadersEntry"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(
            set: &'a QList_QHttpHeadersEntry,
            pos: isize,
        ) -> &'a QHttpHeadersEntry;
        #[rust_name = "index_of_QHttpHeadersEntry"]
        fn qlistIndexOf(_: &QList_QHttpHeadersEntry, _: &QHttpHeadersEntry) -> isize;
        #[rust_name = "insert_QHttpHeadersEntry"]
        fn qlistInsert(_: &mut QList_QHttpHeadersEntry, _: isize, _: &QHttpHeadersEntry);
        #[rust_name = "len_QHttpHeadersEntry"]
        fn qlistLen(_: &QList_QHttpHeadersEntry) -> isize;
        #[rust_name = "remove_QHttpHeadersEntry"]
        fn qlistRemove(_: &mut QList_QHttpHeadersEntry, _: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QHttpHeadersEntry) {
    ffi::cxx_clear_qlist_qhttpheadersentry(v);
}

pub(crate) fn contains(v: &ffi::QList_QHttpHeadersEntry, item: &ffi::QHttpHeadersEntry) -> bool {
    ffi::cxx_contains(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QHttpHeadersEntry, size: isize) {
    ffi::reserve_QHttpHeadersEntry(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QHttpHeadersEntry, value: &QHttpHeadersEntry) {
    ffi::append_QHttpHeadersEntry(v, value);
}

pub(crate) fn clone(v: &ffi::QList_QHttpHeadersEntry) -> ffi::QList_QHttpHeadersEntry {
    ffi::qlist_clone_QHttpHeadersEntry(v)
}

pub(crate) fn default() -> ffi::QList_QHttpHeadersEntry {
    ffi::qlist_default_QHttpHeadersEntry()
}

pub(crate) fn drop(v: &mut ffi::QList_QHttpHeadersEntry) {
    ffi::qlist_drop_QHttpHeadersEntry(v);
}

pub(crate) unsafe fn get_unchecked(
    v: &ffi::QList_QHttpHeadersEntry,
    pos: isize,
) -> &QHttpHeadersEntry {
    ffi::get_unchecked_QHttpHeadersEntry(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_QHttpHeadersEntry, value: &QHttpHeadersEntry) -> isize {
    ffi::index_of_QHttpHeadersEntry(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_QHttpHeadersEntry, pos: isize, value: &QHttpHeadersEntry) {
    ffi::insert_QHttpHeadersEntry(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_QHttpHeadersEntry) -> isize {
    ffi::len_QHttpHeadersEntry(v)
}

pub(crate) fn remove(s: &mut ffi::QList_QHttpHeadersEntry, pos: isize) {
    ffi::remove_QHttpHeadersEntry(s, pos);
}
