use crate::{QPair, QPairPair_QByteArray_QByteArray};

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qpair_qbytearray_qbytearray.h");
        type QPair_QByteArray_QByteArray = crate::QPair<crate::QPairPair_QByteArray_QByteArray>;
        include!("cxx-qt-io/qlist_qpair_qbytearray_qbytearray.h");
        type QList_QPair_QByteArray_QByteArray = cxx_qt_lib::QList<QPair_QByteArray_QByteArray>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_qpair_qbytearray_qbytearray"]
        fn qlistClear(list: &mut QList_QPair_QByteArray_QByteArray);
        #[rust_name = "cxx_contains"]
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
        #[rust_name = "reserve_QPair_QByteArray_QByteArray"]
        fn qlistReserve(_: &mut QList_QPair_QByteArray_QByteArray, size: isize);
        #[rust_name = "append_QPair_QByteArray_QByteArray"]
        fn qlistAppend(_: &mut QList_QPair_QByteArray_QByteArray, _: &QPair_QByteArray_QByteArray);
        #[rust_name = "get_unchecked_QPair_QByteArray_QByteArray"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(
            set: &'a QList_QPair_QByteArray_QByteArray,
            pos: isize,
        ) -> &'a QPair_QByteArray_QByteArray;
        #[rust_name = "index_of_QPair_QByteArray_QByteArray"]
        fn qlistIndexOf(
            _: &QList_QPair_QByteArray_QByteArray,
            _: &QPair_QByteArray_QByteArray,
        ) -> isize;
        #[rust_name = "insert_QPair_QByteArray_QByteArray"]
        fn qlistInsert(
            _: &mut QList_QPair_QByteArray_QByteArray,
            _: isize,
            _: &QPair_QByteArray_QByteArray,
        );
        #[rust_name = "len_QPair_QByteArray_QByteArray"]
        fn qlistLen(_: &QList_QPair_QByteArray_QByteArray) -> isize;
        #[rust_name = "remove_QPair_QByteArray_QByteArray"]
        fn qlistRemove(_: &mut QList_QPair_QByteArray_QByteArray, _: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QPair_QByteArray_QByteArray) {
    ffi::cxx_clear_qlist_qpair_qbytearray_qbytearray(v);
}

pub(crate) fn contains(
    v: &ffi::QList_QPair_QByteArray_QByteArray,
    item: &ffi::QPair_QByteArray_QByteArray,
) -> bool {
    ffi::cxx_contains(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QPair_QByteArray_QByteArray, size: isize) {
    ffi::reserve_QPair_QByteArray_QByteArray(v, size);
}

pub(crate) fn append(
    v: &mut ffi::QList_QPair_QByteArray_QByteArray,
    value: &QPair<QPairPair_QByteArray_QByteArray>,
) {
    ffi::append_QPair_QByteArray_QByteArray(v, value);
}

pub(crate) fn clone(
    v: &ffi::QList_QPair_QByteArray_QByteArray,
) -> ffi::QList_QPair_QByteArray_QByteArray {
    ffi::qlist_clone_QPair_QByteArray_QByteArray(v)
}

pub(crate) fn default() -> ffi::QList_QPair_QByteArray_QByteArray {
    ffi::qlist_default_QPair_QByteArray_QByteArray()
}

pub(crate) fn drop(v: &mut ffi::QList_QPair_QByteArray_QByteArray) {
    ffi::qlist_drop_QPair_QByteArray_QByteArray(v);
}

pub(crate) unsafe fn get_unchecked(
    v: &ffi::QList_QPair_QByteArray_QByteArray,
    pos: isize,
) -> &QPair<QPairPair_QByteArray_QByteArray> {
    ffi::get_unchecked_QPair_QByteArray_QByteArray(v, pos)
}

pub(crate) fn index_of(
    v: &ffi::QList_QPair_QByteArray_QByteArray,
    value: &QPair<QPairPair_QByteArray_QByteArray>,
) -> isize {
    ffi::index_of_QPair_QByteArray_QByteArray(v, value)
}

pub(crate) fn insert(
    v: &mut ffi::QList_QPair_QByteArray_QByteArray,
    pos: isize,
    value: &QPair<QPairPair_QByteArray_QByteArray>,
) {
    ffi::insert_QPair_QByteArray_QByteArray(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_QPair_QByteArray_QByteArray) -> isize {
    ffi::len_QPair_QByteArray_QByteArray(v)
}

pub(crate) fn remove(s: &mut ffi::QList_QPair_QByteArray_QByteArray, pos: isize) {
    ffi::remove_QPair_QByteArray_QByteArray(s, pos);
}
