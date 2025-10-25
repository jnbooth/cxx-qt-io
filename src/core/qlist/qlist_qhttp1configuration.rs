//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qhttp1configuration.h");
        type QHttp1Configuration = crate::QHttp1Configuration;

        include!("cxx-qt-io/qlist_qhttp1configuration.h");
        type QList_QHttp1Configuration = cxx_qt_lib::QList<QHttp1Configuration>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_clear_QHttp1Configuration"]
        fn qlistClear(list: &mut QList_QHttp1Configuration);
        #[rust_name = "qlist_contains_QHttp1Configuration"]
        fn qlistContains(list: &QList_QHttp1Configuration, _: &QHttp1Configuration) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QHttp1Configuration"]
        fn construct(_: &QList_QHttp1Configuration) -> QList_QHttp1Configuration;
        #[rust_name = "qlist_default_QHttp1Configuration"]
        fn construct() -> QList_QHttp1Configuration;
        #[rust_name = "qlist_drop_QHttp1Configuration"]
        fn drop(_: &mut QList_QHttp1Configuration);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "qlist_reserve_QHttp1Configuration"]
        fn qlistReserve(_: &mut QList_QHttp1Configuration, size: isize);
        #[rust_name = "qlist_append_QHttp1Configuration"]
        fn qlistAppend(_: &mut QList_QHttp1Configuration, _: &QHttp1Configuration);
        #[rust_name = "qlist_get_unchecked_QHttp1Configuration"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QHttp1Configuration,
            pos: isize,
        ) -> &QHttp1Configuration;
        #[rust_name = "qlist_index_of_QHttp1Configuration"]
        fn qlistIndexOf(_: &QList_QHttp1Configuration, _: &QHttp1Configuration) -> isize;
        #[rust_name = "qlist_insert_QHttp1Configuration"]
        fn qlistInsert(_: &mut QList_QHttp1Configuration, _: isize, _: &QHttp1Configuration);
        #[rust_name = "qlist_remove_QHttp1Configuration"]
        fn qlistRemove(_: &mut QList_QHttp1Configuration, _: isize);
        #[rust_name = "qlist_len_QHttp1Configuration"]
        fn qlistLen(_: &QList_QHttp1Configuration) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QHttp1Configuration) {
    ffi::qlist_clear_QHttp1Configuration(v);
}

pub(crate) fn contains(
    v: &ffi::QList_QHttp1Configuration,
    item: &ffi::QHttp1Configuration,
) -> bool {
    ffi::qlist_contains_QHttp1Configuration(v, item)
}

pub(crate) fn reserve(v: &mut ffi::QList_QHttp1Configuration, size: isize) {
    ffi::qlist_reserve_QHttp1Configuration(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_QHttp1Configuration, value: &ffi::QHttp1Configuration) {
    ffi::qlist_append_QHttp1Configuration(v, value);
}

pub(crate) fn clone(s: &ffi::QList_QHttp1Configuration) -> ffi::QList_QHttp1Configuration {
    ffi::qlist_clone_QHttp1Configuration(s)
}

pub(crate) fn default() -> ffi::QList_QHttp1Configuration {
    ffi::qlist_default_QHttp1Configuration()
}

pub(crate) fn drop(s: &mut ffi::QList_QHttp1Configuration) {
    ffi::qlist_drop_QHttp1Configuration(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QHttp1Configuration,
    pos: isize,
) -> &ffi::QHttp1Configuration {
    unsafe { ffi::qlist_get_unchecked_QHttp1Configuration(s, pos) }
}

pub(crate) fn index_of(
    v: &ffi::QList_QHttp1Configuration,
    value: &ffi::QHttp1Configuration,
) -> isize {
    ffi::qlist_index_of_QHttp1Configuration(v, value)
}

pub(crate) fn insert(
    s: &mut ffi::QList_QHttp1Configuration,
    pos: isize,
    value: &ffi::QHttp1Configuration,
) {
    ffi::qlist_insert_QHttp1Configuration(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QHttp1Configuration) -> isize {
    ffi::qlist_len_QHttp1Configuration(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QHttp1Configuration, pos: isize) {
    ffi::qlist_remove_QHttp1Configuration(s, pos);
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
