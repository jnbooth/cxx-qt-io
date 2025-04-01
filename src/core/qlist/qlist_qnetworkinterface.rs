//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qnetworkinterface.h");
        type QNetworkInterface = crate::QNetworkInterface;

        include!("cxx-qt-io/qlist_qnetworkinterface.h");
        type QList_QNetworkInterface = cxx_qt_lib::QList<QNetworkInterface>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_QNetworkInterface"]
        fn qlistClear(list: &mut QList_QNetworkInterface);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QNetworkInterface"]
        fn construct(_: &QList_QNetworkInterface) -> QList_QNetworkInterface;
        #[rust_name = "qlist_default_QNetworkInterface"]
        fn construct() -> QList_QNetworkInterface;
        #[rust_name = "qlist_drop_QNetworkInterface"]
        fn drop(_: &mut QList_QNetworkInterface);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QNetworkInterface"]
        fn qlistReserve(_: &mut QList_QNetworkInterface, size: isize);
        #[rust_name = "append_QNetworkInterface"]
        fn qlistAppend(_: &mut QList_QNetworkInterface, _: &QNetworkInterface);
        #[rust_name = "get_unchecked_QNetworkInterface"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QNetworkInterface,
            pos: isize,
        ) -> &QNetworkInterface;
        #[rust_name = "insert_QNetworkInterface"]
        fn qlistInsert(_: &mut QList_QNetworkInterface, _: isize, _: &QNetworkInterface);
        #[rust_name = "remove_QNetworkInterface"]
        fn qlistRemove(_: &mut QList_QNetworkInterface, _: isize);
        #[rust_name = "len_QNetworkInterface"]
        fn qlistLen(_: &QList_QNetworkInterface) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QNetworkInterface) {
    ffi::cxx_clear_qlist_QNetworkInterface(v);
}

pub(crate) fn contains(_: &ffi::QList_QNetworkInterface, _: &ffi::QNetworkInterface) -> bool {
    false
}

pub(crate) fn reserve(v: &mut ffi::QList_QNetworkInterface, size: isize) {
    ffi::reserve_QNetworkInterface(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QNetworkInterface, value: &ffi::QNetworkInterface) {
    ffi::append_QNetworkInterface(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QNetworkInterface) -> ffi::QList_QNetworkInterface {
    ffi::qlist_clone_QNetworkInterface(s)
}

pub(crate) fn default() -> ffi::QList_QNetworkInterface {
    ffi::qlist_default_QNetworkInterface()
}

pub(crate) fn drop(s: &mut ffi::QList_QNetworkInterface) {
    ffi::qlist_drop_QNetworkInterface(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QNetworkInterface,
    pos: isize,
) -> &ffi::QNetworkInterface {
    ffi::get_unchecked_QNetworkInterface(s, pos)
}

pub(crate) fn index_of(_: &ffi::QList_QNetworkInterface, _: &ffi::QNetworkInterface) -> isize {
    -1
}

pub(crate) fn insert(
    s: &mut ffi::QList_QNetworkInterface,
    pos: isize,
    value: &ffi::QNetworkInterface,
) {
    ffi::insert_QNetworkInterface(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QNetworkInterface) -> isize {
    ffi::len_QNetworkInterface(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QNetworkInterface, pos: isize) {
    ffi::remove_QNetworkInterface(s, pos);
}
