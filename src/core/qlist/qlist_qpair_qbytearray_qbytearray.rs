//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qpair_qbytearray_qbytearray.h");
        type QPair_QByteArray_QByteArray =
            crate::QPair<cxx_qt_lib::QByteArray, cxx_qt_lib::QByteArray>;

        include!("cxx-qt-io/qlist_qpair_qbytearray_qbytearray.h");
        type QList_QPair_QByteArray_QByteArray = cxx_qt_lib::QList<QPair_QByteArray_QByteArray>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QPair_QByteArray_QByteArray"]
        fn qlistClear(list: &mut QList_QPair_QByteArray_QByteArray);
        #[rust_name = "qlist_contains_QPair_QByteArray_QByteArray"]
        fn qlistContains(
            list: &QList_QPair_QByteArray_QByteArray,
            _: &QPair_QByteArray_QByteArray,
        ) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QPair_QByteArray_QByteArray"]
        fn construct(_: &QList_QPair_QByteArray_QByteArray) -> QList_QPair_QByteArray_QByteArray;
        #[rust_name = "qlist_default_QPair_QByteArray_QByteArray"]
        fn construct() -> QList_QPair_QByteArray_QByteArray;
        #[rust_name = "qlist_drop_QPair_QByteArray_QByteArray"]
        fn drop(_: &mut QList_QPair_QByteArray_QByteArray);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QPair_QByteArray_QByteArray"]
        fn qlistReserve(_: &mut QList_QPair_QByteArray_QByteArray, size: isize);
        #[rust_name = "qlist_append_QPair_QByteArray_QByteArray"]
        fn qlistAppend(_: &mut QList_QPair_QByteArray_QByteArray, _: &QPair_QByteArray_QByteArray);
        #[rust_name = "qlist_get_unchecked_QPair_QByteArray_QByteArray"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QPair_QByteArray_QByteArray,
            pos: isize,
        ) -> &QPair_QByteArray_QByteArray;
        #[rust_name = "qlist_index_of_QPair_QByteArray_QByteArray"]
        fn qlistIndexOf(
            _: &QList_QPair_QByteArray_QByteArray,
            _: &QPair_QByteArray_QByteArray,
        ) -> isize;
        #[rust_name = "qlist_insert_QPair_QByteArray_QByteArray"]
        fn qlistInsert(
            _: &mut QList_QPair_QByteArray_QByteArray,
            _: isize,
            _: &QPair_QByteArray_QByteArray,
        );
        #[rust_name = "qlist_remove_QPair_QByteArray_QByteArray"]
        fn qlistRemove(_: &mut QList_QPair_QByteArray_QByteArray, _: isize);
        #[rust_name = "qlist_len_QPair_QByteArray_QByteArray"]
        fn qlistLen(_: &QList_QPair_QByteArray_QByteArray) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QPair_QByteArray_QByteArray) {
    ffi::qlist_clear_QPair_QByteArray_QByteArray(v);
}

pub(crate) fn contains(
    v: &ffi::QList_QPair_QByteArray_QByteArray,
    item: &ffi::QPair_QByteArray_QByteArray,
) -> bool {
    ffi::qlist_contains_QPair_QByteArray_QByteArray(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QPair_QByteArray_QByteArray, size: isize) {
    ffi::qlist_reserve_QPair_QByteArray_QByteArray(v, size);
}

pub(crate) fn append(
    v: &mut ffi::QList_QPair_QByteArray_QByteArray,
    value: &ffi::QPair_QByteArray_QByteArray,
) {
    ffi::qlist_append_QPair_QByteArray_QByteArray(v, value);
}

pub(crate) fn clone(
    s: &ffi::QList_QPair_QByteArray_QByteArray,
) -> ffi::QList_QPair_QByteArray_QByteArray {
    ffi::qlist_clone_QPair_QByteArray_QByteArray(s)
}

pub(crate) fn default() -> ffi::QList_QPair_QByteArray_QByteArray {
    ffi::qlist_default_QPair_QByteArray_QByteArray()
}

pub(crate) fn drop(s: &mut ffi::QList_QPair_QByteArray_QByteArray) {
    ffi::qlist_drop_QPair_QByteArray_QByteArray(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QPair_QByteArray_QByteArray,
    pos: isize,
) -> &ffi::QPair_QByteArray_QByteArray {
    unsafe { ffi::qlist_get_unchecked_QPair_QByteArray_QByteArray(s, pos) }
}

pub(crate) fn index_of(
    v: &ffi::QList_QPair_QByteArray_QByteArray,
    value: &ffi::QPair_QByteArray_QByteArray,
) -> isize {
    ffi::qlist_index_of_QPair_QByteArray_QByteArray(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QPair_QByteArray_QByteArray,
    pos: isize,
    value: &ffi::QPair_QByteArray_QByteArray,
) {
    ffi::qlist_insert_QPair_QByteArray_QByteArray(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QPair_QByteArray_QByteArray) -> isize {
    ffi::qlist_len_QPair_QByteArray_QByteArray(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QPair_QByteArray_QByteArray, pos: isize) {
    ffi::qlist_remove_QPair_QByteArray_QByteArray(s, pos);
}
