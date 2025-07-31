//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qset/generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-io/qssldiffiehellmanparameters.h");
        type QSslDiffieHellmanParameters = crate::QSslDiffieHellmanParameters;

        include!("cxx-qt-io/qset_qssldiffiehellmanparameters.h");
        type QSet_QSslDiffieHellmanParameters = cxx_qt_lib::QSet<QSslDiffieHellmanParameters>;
    }

    #[namespace = "rust::cxxqtio1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_clear_QSslDiffieHellmanParameters"]
        fn qsetClear(set: &mut QSet_QSslDiffieHellmanParameters);
        #[rust_name = "qset_contains_QSslDiffieHellmanParameters"]
        fn qsetContains(
            set: &QSet_QSslDiffieHellmanParameters,
            _: &QSslDiffieHellmanParameters,
        ) -> bool;
        #[rust_name = "qset_remove_QSslDiffieHellmanParameters"]
        fn qsetRemove(
            set: &mut QSet_QSslDiffieHellmanParameters,
            _: &QSslDiffieHellmanParameters,
        ) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_QSslDiffieHellmanParameters"]
        fn construct(_: &QSet_QSslDiffieHellmanParameters) -> QSet_QSslDiffieHellmanParameters;
        #[rust_name = "qset_default_QSslDiffieHellmanParameters"]
        fn construct() -> QSet_QSslDiffieHellmanParameters;
        #[rust_name = "qset_drop_QSslDiffieHellmanParameters"]
        fn drop(_: &mut QSet_QSslDiffieHellmanParameters);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_get_unchecked_QSslDiffieHellmanParameters"]
        unsafe fn qsetGetUnchecked(
            set: &QSet_QSslDiffieHellmanParameters,
            pos: isize,
        ) -> &QSslDiffieHellmanParameters;
        #[rust_name = "qset_insert_QSslDiffieHellmanParameters"]
        fn qsetInsert(_: &mut QSet_QSslDiffieHellmanParameters, _: &QSslDiffieHellmanParameters);
        #[rust_name = "qset_len_QSslDiffieHellmanParameters"]
        fn qsetLen(_: &QSet_QSslDiffieHellmanParameters) -> isize;
        #[rust_name = "qset_reserve_QSslDiffieHellmanParameters"]
        fn qsetReserve(_: &mut QSet_QSslDiffieHellmanParameters, size: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QSet_QSslDiffieHellmanParameters) {
    ffi::qset_clear_QSslDiffieHellmanParameters(v);
}

pub(crate) fn contains(
    v: &ffi::QSet_QSslDiffieHellmanParameters,
    item: &ffi::QSslDiffieHellmanParameters,
) -> bool {
    ffi::qset_contains_QSslDiffieHellmanParameters(v, item)
}

pub(crate) fn remove(
    v: &mut ffi::QSet_QSslDiffieHellmanParameters,
    item: &ffi::QSslDiffieHellmanParameters,
) -> bool {
    ffi::qset_remove_QSslDiffieHellmanParameters(v, item)
}

pub(crate) fn clone(
    s: &ffi::QSet_QSslDiffieHellmanParameters,
) -> ffi::QSet_QSslDiffieHellmanParameters {
    ffi::qset_clone_QSslDiffieHellmanParameters(s)
}

pub(crate) fn default() -> ffi::QSet_QSslDiffieHellmanParameters {
    ffi::qset_default_QSslDiffieHellmanParameters()
}

pub(crate) fn drop(s: &mut ffi::QSet_QSslDiffieHellmanParameters) {
    ffi::qset_drop_QSslDiffieHellmanParameters(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QSet_QSslDiffieHellmanParameters,
    pos: isize,
) -> &ffi::QSslDiffieHellmanParameters {
    unsafe { ffi::qset_get_unchecked_QSslDiffieHellmanParameters(s, pos) }
}

pub(crate) fn insert(
    s: &mut ffi::QSet_QSslDiffieHellmanParameters,
    value: &ffi::QSslDiffieHellmanParameters,
) {
    ffi::qset_insert_QSslDiffieHellmanParameters(s, value);
}

pub(crate) fn len(s: &ffi::QSet_QSslDiffieHellmanParameters) -> isize {
    ffi::qset_len_QSslDiffieHellmanParameters(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_QSslDiffieHellmanParameters, size: isize) {
    ffi::qset_reserve_QSslDiffieHellmanParameters(s, size);
}
