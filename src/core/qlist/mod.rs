use cxx::type_id;
use cxx_qt_lib::{QList, QListElement};

macro_rules! impl_qlist_element {
    ( $typeName:ty, $module:ident, $typeId:literal ) => {
        mod $module;

        impl QListElement for $typeName {
            type TypeId = type_id!($typeId);

            fn append(list: &mut QList<Self>, value: Self) {
                $module::append(list, &value);
            }

            fn append_clone(list: &mut QList<Self>, value: &Self) {
                $module::append(list, value);
            }

            fn clear(list: &mut QList<Self>) {
                $module::clear(list)
            }

            fn clone(list: &QList<Self>) -> QList<Self> {
                $module::clone(list)
            }

            fn contains(list: &QList<Self>, value: &Self) -> bool {
                $module::contains(list, value)
            }

            fn default() -> QList<Self> {
                $module::default()
            }

            fn drop(list: &mut QList<Self>) {
                $module::drop(list);
            }

            unsafe fn get_unchecked(list: &QList<Self>, pos: isize) -> &Self {
                $module::get_unchecked(list, pos)
            }

            fn index_of(list: &QList<Self>, value: &Self) -> isize {
                $module::index_of(list, value)
            }

            fn insert(list: &mut QList<Self>, pos: isize, value: Self) {
                $module::insert(list, pos, &value);
            }

            fn insert_clone(list: &mut QList<Self>, pos: isize, value: &Self) {
                $module::insert(list, pos, value);
            }

            fn len(list: &QList<Self>) -> isize {
                $module::len(list)
            }

            fn remove(list: &mut QList<Self>, pos: isize) {
                $module::remove(list, pos);
            }

            fn reserve(list: &mut QList<Self>, size: isize) {
                $module::reserve(list, size);
            }
        }
    };
}

impl_qlist_element!(
    crate::QPair<crate::QPairPair_QByteArray_QByteArray>,
    qlist_qpair_qbytearray_qbytearray,
    "QList_QPair_QByteArray_QByteArray"
);

impl_qlist_element!(
    crate::QHostAddress,
    qlist_qhostaddress,
    "QList_QHostAddress"
);

impl_qlist_element!(
    crate::QNetworkAddressEntry,
    qlist_qnetworkaddressentry,
    "QList_QNetworkAddressEntry"
);

impl_qlist_element!(
    crate::QNetworkInterface,
    qlist_qnetworkinterface,
    "QList_QNetworkInterface"
);
