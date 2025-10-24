//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qnetworkcachemetadata.h");
        type QNetworkCacheMetaData = crate::QNetworkCacheMetaData;

        include!("cxx-qt-io/qlist_qnetworkcachemetadata.h");
        type QList_QNetworkCacheMetaData = cxx_qt_lib::QList<QNetworkCacheMetaData>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QNetworkCacheMetaData"]
        fn qlistClear(list: &mut QList_QNetworkCacheMetaData);
        #[rust_name = "qlist_contains_QNetworkCacheMetaData"]
        fn qlistContains(list: &QList_QNetworkCacheMetaData, _: &QNetworkCacheMetaData) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QNetworkCacheMetaData"]
        fn construct(_: &QList_QNetworkCacheMetaData) -> QList_QNetworkCacheMetaData;
        #[rust_name = "qlist_default_QNetworkCacheMetaData"]
        fn construct() -> QList_QNetworkCacheMetaData;
        #[rust_name = "qlist_drop_QNetworkCacheMetaData"]
        fn drop(_: &mut QList_QNetworkCacheMetaData);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QNetworkCacheMetaData"]
        fn qlistReserve(_: &mut QList_QNetworkCacheMetaData, size: isize);
        #[rust_name = "qlist_append_QNetworkCacheMetaData"]
        fn qlistAppend(_: &mut QList_QNetworkCacheMetaData, _: &QNetworkCacheMetaData);
        #[rust_name = "qlist_get_unchecked_QNetworkCacheMetaData"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QNetworkCacheMetaData,
            pos: isize,
        ) -> &QNetworkCacheMetaData;
        #[rust_name = "qlist_index_of_QNetworkCacheMetaData"]
        fn qlistIndexOf(_: &QList_QNetworkCacheMetaData, _: &QNetworkCacheMetaData) -> isize;
        #[rust_name = "qlist_insert_QNetworkCacheMetaData"]
        fn qlistInsert(_: &mut QList_QNetworkCacheMetaData, _: isize, _: &QNetworkCacheMetaData);
        #[rust_name = "qlist_remove_QNetworkCacheMetaData"]
        fn qlistRemove(_: &mut QList_QNetworkCacheMetaData, _: isize);
        #[rust_name = "qlist_len_QNetworkCacheMetaData"]
        fn qlistLen(_: &QList_QNetworkCacheMetaData) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QNetworkCacheMetaData) {
    ffi::qlist_clear_QNetworkCacheMetaData(v);
}

pub(crate) fn contains(
    v: &ffi::QList_QNetworkCacheMetaData,
    item: &ffi::QNetworkCacheMetaData,
) -> bool {
    ffi::qlist_contains_QNetworkCacheMetaData(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QNetworkCacheMetaData, size: isize) {
    ffi::qlist_reserve_QNetworkCacheMetaData(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QNetworkCacheMetaData, value: &ffi::QNetworkCacheMetaData) {
    ffi::qlist_append_QNetworkCacheMetaData(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QNetworkCacheMetaData) -> ffi::QList_QNetworkCacheMetaData {
    ffi::qlist_clone_QNetworkCacheMetaData(s)
}

pub(crate) fn default() -> ffi::QList_QNetworkCacheMetaData {
    ffi::qlist_default_QNetworkCacheMetaData()
}

pub(crate) fn drop(s: &mut ffi::QList_QNetworkCacheMetaData) {
    ffi::qlist_drop_QNetworkCacheMetaData(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QNetworkCacheMetaData,
    pos: isize,
) -> &ffi::QNetworkCacheMetaData {
    unsafe { ffi::qlist_get_unchecked_QNetworkCacheMetaData(s, pos) }
}

pub(crate) fn index_of(
    v: &ffi::QList_QNetworkCacheMetaData,
    value: &ffi::QNetworkCacheMetaData,
) -> isize {
    ffi::qlist_index_of_QNetworkCacheMetaData(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QNetworkCacheMetaData,
    pos: isize,
    value: &ffi::QNetworkCacheMetaData,
) {
    ffi::qlist_insert_QNetworkCacheMetaData(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QNetworkCacheMetaData) -> isize {
    ffi::qlist_len_QNetworkCacheMetaData(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QNetworkCacheMetaData, pos: isize) {
    ffi::qlist_remove_QNetworkCacheMetaData(s, pos);
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
