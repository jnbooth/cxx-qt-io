//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qnetworkdatagram.h");
        type QNetworkDatagram = crate::QNetworkDatagram;

        include!("cxx-qt-io/qlist_qnetworkdatagram.h");
        type QList_QNetworkDatagram = cxx_qt_lib::QList<QNetworkDatagram>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_QNetworkDatagram"]
        fn qlistClear(list: &mut QList_QNetworkDatagram);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QNetworkDatagram"]
        fn construct(_: &QList_QNetworkDatagram) -> QList_QNetworkDatagram;
        #[rust_name = "qlist_default_QNetworkDatagram"]
        fn construct() -> QList_QNetworkDatagram;
        #[rust_name = "qlist_drop_QNetworkDatagram"]
        fn drop(_: &mut QList_QNetworkDatagram);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QNetworkDatagram"]
        fn qlistReserve(_: &mut QList_QNetworkDatagram, size: isize);
        #[rust_name = "append_QNetworkDatagram"]
        fn qlistAppend(_: &mut QList_QNetworkDatagram, _: &QNetworkDatagram);
        #[rust_name = "get_unchecked_QNetworkDatagram"]
        unsafe fn qlistGetUnchecked(set: &QList_QNetworkDatagram, pos: isize) -> &QNetworkDatagram;
        #[rust_name = "insert_QNetworkDatagram"]
        fn qlistInsert(_: &mut QList_QNetworkDatagram, _: isize, _: &QNetworkDatagram);
        #[rust_name = "remove_QNetworkDatagram"]
        fn qlistRemove(_: &mut QList_QNetworkDatagram, _: isize);
        #[rust_name = "len_QNetworkDatagram"]
        fn qlistLen(_: &QList_QNetworkDatagram) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QNetworkDatagram) {
    ffi::cxx_clear_qlist_QNetworkDatagram(v);
}

pub(crate) fn contains(_: &ffi::QList_QNetworkDatagram, _: &ffi::QNetworkDatagram) -> bool {
    false
}

pub(crate) fn reserve(v: &mut ffi::QList_QNetworkDatagram, size: isize) {
    ffi::reserve_QNetworkDatagram(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QNetworkDatagram, value: &ffi::QNetworkDatagram) {
    ffi::append_QNetworkDatagram(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QNetworkDatagram) -> ffi::QList_QNetworkDatagram {
    ffi::qlist_clone_QNetworkDatagram(s)
}

pub(crate) fn default() -> ffi::QList_QNetworkDatagram {
    ffi::qlist_default_QNetworkDatagram()
}

pub(crate) fn drop(s: &mut ffi::QList_QNetworkDatagram) {
    ffi::qlist_drop_QNetworkDatagram(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QNetworkDatagram,
    pos: isize,
) -> &ffi::QNetworkDatagram {
    ffi::get_unchecked_QNetworkDatagram(s, pos)
}

pub(crate) fn index_of(_: &ffi::QList_QNetworkDatagram, _: &ffi::QNetworkDatagram) -> isize {
    -1
}

pub(crate) fn insert(
    s: &mut ffi::QList_QNetworkDatagram,
    pos: isize,
    value: &ffi::QNetworkDatagram,
) {
    ffi::insert_QNetworkDatagram(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QNetworkDatagram) -> isize {
    ffi::len_QNetworkDatagram(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QNetworkDatagram, pos: isize) {
    ffi::remove_QNetworkDatagram(s, pos);
}
