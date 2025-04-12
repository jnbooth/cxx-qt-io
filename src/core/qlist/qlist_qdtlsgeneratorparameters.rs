//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qdtlsgeneratorparameters.h");
        type QDtlsGeneratorParameters = crate::QDtlsGeneratorParameters;

        include!("cxx-qt-io/qlist_qdtlsgeneratorparameters.h");
        type QList_QDtlsGeneratorParameters = cxx_qt_lib::QList<QDtlsGeneratorParameters>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_QDtlsGeneratorParameters"]
        fn qlistClear(list: &mut QList_QDtlsGeneratorParameters);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QDtlsGeneratorParameters"]
        fn construct(_: &QList_QDtlsGeneratorParameters) -> QList_QDtlsGeneratorParameters;
        #[rust_name = "qlist_default_QDtlsGeneratorParameters"]
        fn construct() -> QList_QDtlsGeneratorParameters;
        #[rust_name = "qlist_drop_QDtlsGeneratorParameters"]
        fn drop(_: &mut QList_QDtlsGeneratorParameters);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QDtlsGeneratorParameters"]
        fn qlistReserve(_: &mut QList_QDtlsGeneratorParameters, size: isize);
        #[rust_name = "append_QDtlsGeneratorParameters"]
        fn qlistAppend(_: &mut QList_QDtlsGeneratorParameters, _: &QDtlsGeneratorParameters);
        #[rust_name = "get_unchecked_QDtlsGeneratorParameters"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QDtlsGeneratorParameters,
            pos: isize,
        ) -> &QDtlsGeneratorParameters;
        #[rust_name = "insert_QDtlsGeneratorParameters"]
        fn qlistInsert(
            _: &mut QList_QDtlsGeneratorParameters,
            _: isize,
            _: &QDtlsGeneratorParameters,
        );
        #[rust_name = "remove_QDtlsGeneratorParameters"]
        fn qlistRemove(_: &mut QList_QDtlsGeneratorParameters, _: isize);
        #[rust_name = "len_QDtlsGeneratorParameters"]
        fn qlistLen(_: &QList_QDtlsGeneratorParameters) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QDtlsGeneratorParameters) {
    ffi::cxx_clear_qlist_QDtlsGeneratorParameters(v);
}

pub(crate) fn contains(
    _: &ffi::QList_QDtlsGeneratorParameters,
    _: &ffi::QDtlsGeneratorParameters,
) -> bool {
    false
}

pub(crate) fn reserve(v: &mut ffi::QList_QDtlsGeneratorParameters, size: isize) {
    ffi::reserve_QDtlsGeneratorParameters(v, size);
}

pub(crate) fn append(
    v: &mut ffi::QList_QDtlsGeneratorParameters,
    value: &ffi::QDtlsGeneratorParameters,
) {
    ffi::append_QDtlsGeneratorParameters(v, value);
}

pub(crate) fn clone(
    s: &ffi::QList_QDtlsGeneratorParameters,
) -> ffi::QList_QDtlsGeneratorParameters {
    ffi::qlist_clone_QDtlsGeneratorParameters(s)
}

pub(crate) fn default() -> ffi::QList_QDtlsGeneratorParameters {
    ffi::qlist_default_QDtlsGeneratorParameters()
}

pub(crate) fn drop(s: &mut ffi::QList_QDtlsGeneratorParameters) {
    ffi::qlist_drop_QDtlsGeneratorParameters(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QDtlsGeneratorParameters,
    pos: isize,
) -> &ffi::QDtlsGeneratorParameters {
    ffi::get_unchecked_QDtlsGeneratorParameters(s, pos)
}

pub(crate) fn index_of(
    _: &ffi::QList_QDtlsGeneratorParameters,
    _: &ffi::QDtlsGeneratorParameters,
) -> isize {
    -1
}

pub(crate) fn insert(
    s: &mut ffi::QList_QDtlsGeneratorParameters,
    pos: isize,
    value: &ffi::QDtlsGeneratorParameters,
) {
    ffi::insert_QDtlsGeneratorParameters(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QDtlsGeneratorParameters) -> isize {
    ffi::len_QDtlsGeneratorParameters(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QDtlsGeneratorParameters, pos: isize) {
    ffi::remove_QDtlsGeneratorParameters(s, pos);
}
