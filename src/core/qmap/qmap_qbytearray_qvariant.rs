use cxx::{type_id, ExternType};

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-io/qmap_qbytearray_qvariant.h");
        type QMap_QByteArray_QVariant = cxx_qt_lib::QMap<super::QMapPair_QByteArray_QVariant>;
    }

    #[namespace = "rust::cxxqtio1::qmap"]
    unsafe extern "C++" {
        #[rust_name = "qmap_clear_QByteArray_QVariant"]
        fn qmapClear(map: &mut QMap_QByteArray_QVariant);
        #[rust_name = "qmap_contains_QByteArray_QVariant"]
        fn qmapContains(list: &QMap_QByteArray_QVariant, _: &QByteArray) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qmap_clone_QByteArray_QVariant"]
        fn construct(_: &QMap_QByteArray_QVariant) -> QMap_QByteArray_QVariant;
        #[rust_name = "qmap_default_QByteArray_QVariant"]
        fn construct() -> QMap_QByteArray_QVariant;
        #[rust_name = "qmap_drop_QByteArray_QVariant"]
        fn drop(_: &mut QMap_QByteArray_QVariant);
    }

    #[namespace = "rust::cxxqtlib1::qmap"]
    unsafe extern "C++" {
        #[rust_name = "qmap_get_or_default_QByteArray_QVariant"]
        fn qmapGetOrDefault(_: &QMap_QByteArray_QVariant, key: &QByteArray) -> QVariant;
        #[rust_name = "qmap_get_unchecked_key_QByteArray_QVariant"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qmapGetUncheckedKey<'a>(
            _: &'a QMap_QByteArray_QVariant,
            pos: isize,
        ) -> &'a QByteArray;
        #[rust_name = "qmap_get_unchecked_value_QByteArray_QVariant"]
        unsafe fn qmapGetUncheckedValue(_: &QMap_QByteArray_QVariant, pos: isize) -> &QVariant;
        #[rust_name = "qmap_insert_QByteArray_QVariant"]
        fn qmapInsert(_: &mut QMap_QByteArray_QVariant, key: &QByteArray, value: &QVariant);
        #[rust_name = "qmap_len_QByteArray_QVariant"]
        fn qmapLen(_: &QMap_QByteArray_QVariant) -> isize;
        #[rust_name = "qmap_remove_QByteArray_QVariant"]
        fn qmapRemove(_: &mut QMap_QByteArray_QVariant, key: &QByteArray) -> bool;
    }
}

pub(crate) fn clear(v: &mut ffi::QMap_QByteArray_QVariant) {
    ffi::qmap_clear_QByteArray_QVariant(v);
}

pub(crate) fn contains(v: &ffi::QMap_QByteArray_QVariant, item: &ffi::QByteArray) -> bool {
    ffi::qmap_contains_QByteArray_QVariant(v, item)
}

pub(crate) fn clone(map: &ffi::QMap_QByteArray_QVariant) -> ffi::QMap_QByteArray_QVariant {
    ffi::qmap_clone_QByteArray_QVariant(map)
}

pub(crate) fn default() -> ffi::QMap_QByteArray_QVariant {
    ffi::qmap_default_QByteArray_QVariant()
}

pub(crate) fn drop(map: &mut ffi::QMap_QByteArray_QVariant) {
    ffi::qmap_drop_QByteArray_QVariant(map);
}

pub(crate) fn get_or_default(
    map: &ffi::QMap_QByteArray_QVariant,
    key: &ffi::QByteArray,
) -> ffi::QVariant {
    ffi::qmap_get_or_default_QByteArray_QVariant(map, key)
}

pub(crate) unsafe fn get_unchecked_key(
    map: &ffi::QMap_QByteArray_QVariant,
    pos: isize,
) -> &ffi::QByteArray {
    ffi::qmap_get_unchecked_key_QByteArray_QVariant(map, pos)
}

pub(crate) unsafe fn get_unchecked_value(
    map: &ffi::QMap_QByteArray_QVariant,
    pos: isize,
) -> &ffi::QVariant {
    ffi::qmap_get_unchecked_value_QByteArray_QVariant(map, pos)
}

pub(crate) fn insert(
    map: &mut ffi::QMap_QByteArray_QVariant,
    key: &ffi::QByteArray,
    value: &ffi::QVariant,
) {
    ffi::qmap_insert_QByteArray_QVariant(map, key, value);
}

pub(crate) fn len(map: &ffi::QMap_QByteArray_QVariant) -> isize {
    ffi::qmap_len_QByteArray_QVariant(map)
}

pub(crate) fn remove(map: &mut ffi::QMap_QByteArray_QVariant, key: &ffi::QByteArray) -> bool {
    ffi::qmap_remove_QByteArray_QVariant(map, key)
}

#[allow(non_camel_case_types)]
pub struct QMapPair_QByteArray_QVariant;

unsafe impl ExternType for QMapPair_QByteArray_QVariant {
    type Id = type_id!("QMapPair_QByteArray_QVariant");
    type Kind = cxx::kind::Trivial;
}
