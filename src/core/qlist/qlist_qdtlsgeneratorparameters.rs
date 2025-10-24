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
        #[rust_name = "qlist_clear_QDtlsGeneratorParameters"]
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
        #[rust_name = "qlist_reserve_QDtlsGeneratorParameters"]
        fn qlistReserve(_: &mut QList_QDtlsGeneratorParameters, size: isize);
        #[rust_name = "qlist_append_QDtlsGeneratorParameters"]
        fn qlistAppend(_: &mut QList_QDtlsGeneratorParameters, _: &QDtlsGeneratorParameters);
        #[rust_name = "qlist_get_unchecked_QDtlsGeneratorParameters"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QDtlsGeneratorParameters,
            pos: isize,
        ) -> &QDtlsGeneratorParameters;
        #[rust_name = "qlist_insert_QDtlsGeneratorParameters"]
        fn qlistInsert(
            _: &mut QList_QDtlsGeneratorParameters,
            _: isize,
            _: &QDtlsGeneratorParameters,
        );
        #[rust_name = "qlist_remove_QDtlsGeneratorParameters"]
        fn qlistRemove(_: &mut QList_QDtlsGeneratorParameters, _: isize);
        #[rust_name = "qlist_len_QDtlsGeneratorParameters"]
        fn qlistLen(_: &QList_QDtlsGeneratorParameters) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QDtlsGeneratorParameters) {
    ffi::qlist_clear_QDtlsGeneratorParameters(v);
}

pub(crate) fn contains(
    _: &ffi::QList_QDtlsGeneratorParameters,
    _: &ffi::QDtlsGeneratorParameters,
) -> bool {
    false
}

pub(crate) fn reserve(v: &mut ffi::QList_QDtlsGeneratorParameters, size: isize) {
    ffi::qlist_reserve_QDtlsGeneratorParameters(v, size);
}

pub(crate) fn append(
    v: &mut ffi::QList_QDtlsGeneratorParameters,
    value: &ffi::QDtlsGeneratorParameters,
) {
    ffi::qlist_append_QDtlsGeneratorParameters(v, value);
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
    unsafe { ffi::qlist_get_unchecked_QDtlsGeneratorParameters(s, pos) }
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
    ffi::qlist_insert_QDtlsGeneratorParameters(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QDtlsGeneratorParameters) -> isize {
    ffi::qlist_len_QDtlsGeneratorParameters(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QDtlsGeneratorParameters, pos: isize) {
    ffi::qlist_remove_QDtlsGeneratorParameters(s, pos);
}

#[cfg(test)]
mod tests {
    #[test]
    fn len() {
        let empty = super::default();
        assert_eq!(super::len(&empty), 0);
        std::mem::drop(empty);
    }
}
