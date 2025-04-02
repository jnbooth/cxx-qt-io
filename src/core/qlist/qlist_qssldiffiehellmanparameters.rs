//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh
#![allow(clippy::trivially_copy_pass_by_ref)]

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qssldiffiehellmanparameters.h");
        type QSslDiffieHellmanParameters = crate::QSslDiffieHellmanParameters;

        include!("cxx-qt-io/qlist_qssldiffiehellmanparameters.h");
        type QList_QSslDiffieHellmanParameters = cxx_qt_lib::QList<QSslDiffieHellmanParameters>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_qlist_clear_QSslDiffieHellmanParameters"]
        fn qlistClear(list: &mut QList_QSslDiffieHellmanParameters);
        #[rust_name = "cxx_qlist_contains_QSslDiffieHellmanParameters"]
        fn qlistContains(
            list: &QList_QSslDiffieHellmanParameters,
            _: &QSslDiffieHellmanParameters,
        ) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSslDiffieHellmanParameters"]
        fn construct(_: &QList_QSslDiffieHellmanParameters) -> QList_QSslDiffieHellmanParameters;
        #[rust_name = "qlist_default_QSslDiffieHellmanParameters"]
        fn construct() -> QList_QSslDiffieHellmanParameters;
        #[rust_name = "qlist_drop_QSslDiffieHellmanParameters"]
        fn drop(_: &mut QList_QSslDiffieHellmanParameters);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSslDiffieHellmanParameters"]
        fn qlistReserve(_: &mut QList_QSslDiffieHellmanParameters, size: isize);
        #[rust_name = "append_QSslDiffieHellmanParameters"]
        fn qlistAppend(_: &mut QList_QSslDiffieHellmanParameters, _: &QSslDiffieHellmanParameters);
        #[rust_name = "get_unchecked_QSslDiffieHellmanParameters"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QSslDiffieHellmanParameters,
            pos: isize,
        ) -> &QSslDiffieHellmanParameters;
        #[rust_name = "index_of_QSslDiffieHellmanParameters"]
        fn qlistIndexOf(
            _: &QList_QSslDiffieHellmanParameters,
            _: &QSslDiffieHellmanParameters,
        ) -> isize;
        #[rust_name = "insert_QSslDiffieHellmanParameters"]
        fn qlistInsert(
            _: &mut QList_QSslDiffieHellmanParameters,
            _: isize,
            _: &QSslDiffieHellmanParameters,
        );
        #[rust_name = "remove_QSslDiffieHellmanParameters"]
        fn qlistRemove(_: &mut QList_QSslDiffieHellmanParameters, _: isize);
        #[rust_name = "len_QSslDiffieHellmanParameters"]
        fn qlistLen(_: &QList_QSslDiffieHellmanParameters) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslDiffieHellmanParameters) {
    ffi::cxx_qlist_clear_QSslDiffieHellmanParameters(v);
}

pub(crate) fn contains(
    v: &ffi::QList_QSslDiffieHellmanParameters,
    item: &ffi::QSslDiffieHellmanParameters,
) -> bool {
    ffi::cxx_qlist_contains_QSslDiffieHellmanParameters(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslDiffieHellmanParameters, size: isize) {
    ffi::reserve_QSslDiffieHellmanParameters(v, size);
}

pub(crate) fn append(
    v: &mut ffi::QList_QSslDiffieHellmanParameters,
    value: &ffi::QSslDiffieHellmanParameters,
) {
    ffi::append_QSslDiffieHellmanParameters(v, value);
}

pub(crate) fn clone(
    s: &ffi::QList_QSslDiffieHellmanParameters,
) -> ffi::QList_QSslDiffieHellmanParameters {
    ffi::qlist_clone_QSslDiffieHellmanParameters(s)
}

pub(crate) fn default() -> ffi::QList_QSslDiffieHellmanParameters {
    ffi::qlist_default_QSslDiffieHellmanParameters()
}

pub(crate) fn drop(s: &mut ffi::QList_QSslDiffieHellmanParameters) {
    ffi::qlist_drop_QSslDiffieHellmanParameters(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QSslDiffieHellmanParameters,
    pos: isize,
) -> &ffi::QSslDiffieHellmanParameters {
    ffi::get_unchecked_QSslDiffieHellmanParameters(s, pos)
}

pub(crate) fn index_of(
    v: &ffi::QList_QSslDiffieHellmanParameters,
    value: &ffi::QSslDiffieHellmanParameters,
) -> isize {
    ffi::index_of_QSslDiffieHellmanParameters(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QSslDiffieHellmanParameters,
    pos: isize,
    value: &ffi::QSslDiffieHellmanParameters,
) {
    ffi::insert_QSslDiffieHellmanParameters(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslDiffieHellmanParameters) -> isize {
    ffi::len_QSslDiffieHellmanParameters(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslDiffieHellmanParameters, pos: isize) {
    ffi::remove_QSslDiffieHellmanParameters(s, pos);
}
