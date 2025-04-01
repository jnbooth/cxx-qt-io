//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qlist/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qsslcertificateextension.h");
        type QSslCertificateExtension = crate::QSslCertificateExtension;

        include!("cxx-qt-io/qlist_qsslcertificateextension.h");
        type QList_QSslCertificateExtension = cxx_qt_lib::QList<QSslCertificateExtension>;
    }

    #[namespace = "rust::cxxqtio1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "cxx_clear_qlist_QSslCertificateExtension"]
        fn qlistClear(list: &mut QList_QSslCertificateExtension);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_QSslCertificateExtension"]
        fn construct(_: &QList_QSslCertificateExtension) -> QList_QSslCertificateExtension;
        #[rust_name = "qlist_default_QSslCertificateExtension"]
        fn construct() -> QList_QSslCertificateExtension;
        #[rust_name = "qlist_drop_QSslCertificateExtension"]
        fn drop(_: &mut QList_QSslCertificateExtension);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_QSslCertificateExtension"]
        fn qlistReserve(_: &mut QList_QSslCertificateExtension, size: isize);
        #[rust_name = "append_QSslCertificateExtension"]
        fn qlistAppend(_: &mut QList_QSslCertificateExtension, _: &QSslCertificateExtension);
        #[rust_name = "get_unchecked_QSslCertificateExtension"]
        unsafe fn qlistGetUnchecked(
            set: &QList_QSslCertificateExtension,
            pos: isize,
        ) -> &QSslCertificateExtension;
        #[rust_name = "insert_QSslCertificateExtension"]
        fn qlistInsert(
            _: &mut QList_QSslCertificateExtension,
            _: isize,
            _: &QSslCertificateExtension,
        );
        #[rust_name = "remove_QSslCertificateExtension"]
        fn qlistRemove(_: &mut QList_QSslCertificateExtension, _: isize);
        #[rust_name = "len_QSslCertificateExtension"]
        fn qlistLen(_: &QList_QSslCertificateExtension) -> isize;
    }
}

pub(crate) fn clear(v: &mut ffi::QList_QSslCertificateExtension) {
    ffi::cxx_clear_qlist_QSslCertificateExtension(v);
}

pub(crate) fn contains(
    _: &ffi::QList_QSslCertificateExtension,
    _: &ffi::QSslCertificateExtension,
) -> bool {
    false
}

pub(crate) fn reserve(v: &mut ffi::QList_QSslCertificateExtension, size: isize) {
    ffi::reserve_QSslCertificateExtension(v, size);
}

pub(crate) fn append(
    v: &mut ffi::QList_QSslCertificateExtension,
    value: &ffi::QSslCertificateExtension,
) {
    ffi::append_QSslCertificateExtension(v, value);
}

pub(crate) fn clone(
    s: &ffi::QList_QSslCertificateExtension,
) -> ffi::QList_QSslCertificateExtension {
    ffi::qlist_clone_QSslCertificateExtension(s)
}

pub(crate) fn default() -> ffi::QList_QSslCertificateExtension {
    ffi::qlist_default_QSslCertificateExtension()
}

pub(crate) fn drop(s: &mut ffi::QList_QSslCertificateExtension) {
    ffi::qlist_drop_QSslCertificateExtension(s);
}

pub(crate) unsafe fn get_unchecked(
    s: &ffi::QList_QSslCertificateExtension,
    pos: isize,
) -> &ffi::QSslCertificateExtension {
    ffi::get_unchecked_QSslCertificateExtension(s, pos)
}

pub(crate) fn index_of(
    _: &ffi::QList_QSslCertificateExtension,
    _: &ffi::QSslCertificateExtension,
) -> isize {
    -1
}

pub(crate) fn insert(
    s: &mut ffi::QList_QSslCertificateExtension,
    pos: isize,
    value: &ffi::QSslCertificateExtension,
) {
    ffi::insert_QSslCertificateExtension(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_QSslCertificateExtension) -> isize {
    ffi::len_QSslCertificateExtension(s)
}

pub(crate) fn remove(s: &mut ffi::QList_QSslCertificateExtension, pos: isize) {
    ffi::remove_QSslCertificateExtension(s, pos);
}
