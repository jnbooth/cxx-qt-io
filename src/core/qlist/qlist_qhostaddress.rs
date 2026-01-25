//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddress = crate::QHostAddress;

        include!("cxx-qt-io/qlist_qhostaddress.h");
        type QList_QHostAddress = cxx_qt_lib::QList<QHostAddress>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QHostAddress"]
        fn qlistClear(list: &mut QList_QHostAddress);
        #[rust_name = "qlist_contains_QHostAddress"]
        fn qlistContains(list: &QList_QHostAddress, _: &QHostAddress) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QHostAddress"]
        fn construct(_: &QList_QHostAddress) -> QList_QHostAddress;
        #[rust_name = "qlist_default_QHostAddress"]
        fn construct() -> QList_QHostAddress;
        #[rust_name = "qlist_drop_QHostAddress"]
        fn drop(_: &mut QList_QHostAddress);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QHostAddress"]
        fn qlistReserve(_: &mut QList_QHostAddress, size: isize);
        #[rust_name = "qlist_append_QHostAddress"]
        fn qlistAppend(_: &mut QList_QHostAddress, _: &QHostAddress);
        #[rust_name = "qlist_get_unchecked_QHostAddress"]
        unsafe fn qlistGetUnchecked(set: &QList_QHostAddress, pos: isize) -> &QHostAddress;
        #[rust_name = "qlist_index_of_QHostAddress"]
        fn qlistIndexOf(_: &QList_QHostAddress, _: &QHostAddress) -> isize;
        #[rust_name = "qlist_insert_QHostAddress"]
        fn qlistInsert(_: &mut QList_QHostAddress, _: isize, _: &QHostAddress);
        #[rust_name = "qlist_remove_QHostAddress"]
        fn qlistRemove(_: &mut QList_QHostAddress, _: isize);
        #[rust_name = "qlist_len_QHostAddress"]
        fn qlistLen(_: &QList_QHostAddress) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QHostAddress) {
    ffi::qlist_clear_QHostAddress(v);
}

pub(crate) fn contains(v: &ffi::QList_QHostAddress, item: &ffi::QHostAddress) -> bool {
    ffi::qlist_contains_QHostAddress(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QHostAddress, size: isize) {
    ffi::qlist_reserve_QHostAddress(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QHostAddress, value: &ffi::QHostAddress) {
    ffi::qlist_append_QHostAddress(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QHostAddress) -> ffi::QList_QHostAddress {
    ffi::qlist_clone_QHostAddress(s)
}

pub(crate) fn default() -> ffi::QList_QHostAddress {
    ffi::qlist_default_QHostAddress()
}

pub(crate) fn drop(s: &mut ffi::QList_QHostAddress) {
    ffi::qlist_drop_QHostAddress(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_QHostAddress, pos: isize) -> &ffi::QHostAddress {
    unsafe { ffi::qlist_get_unchecked_QHostAddress(s, pos) }
}

pub(crate) fn index_of(v: &ffi::QList_QHostAddress, value: &ffi::QHostAddress) -> isize {
    ffi::qlist_index_of_QHostAddress(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_QHostAddress, pos: isize, value: &ffi::QHostAddress) {
    ffi::qlist_insert_QHostAddress(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QHostAddress) -> isize {
    ffi::qlist_len_QHostAddress(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QHostAddress, pos: isize) {
    ffi::qlist_remove_QHostAddress(s, pos);
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
