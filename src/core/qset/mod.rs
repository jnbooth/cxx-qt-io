#![allow(unused)]

use cxx::type_id;
use cxx_qt_lib::{QSet, QSetElement};

macro_rules! impl_qset_element {
    ( $typeName:ident, $module:ident, $typeId:literal $(,)? ) => {
        mod $module;

        impl QSetElement for crate::$typeName {
            type TypeId = type_id!($typeId);

            fn clear(set: &mut QSet<Self>) {
                $module::clear(set)
            }

            fn clone(set: &QSet<Self>) -> QSet<Self> {
                $module::clone(set)
            }

            fn contains(set: &QSet<Self>, value: &Self) -> bool {
                $module::contains(set, value)
            }

            fn default() -> QSet<Self> {
                $module::default()
            }

            fn drop(set: &mut QSet<Self>) {
                $module::drop(set);
            }

            unsafe fn get_unchecked(set: &QSet<Self>, pos: isize) -> &Self {
                $module::get_unchecked(set, pos)
            }

            fn insert(set: &mut QSet<Self>, value: Self) {
                $module::insert(set, &value);
            }

            fn insert_clone(set: &mut QSet<Self>, value: &Self) {
                $module::insert(set, value);
            }

            fn len(set: &QSet<Self>) -> isize {
                $module::len(set)
            }

            fn remove(set: &mut QSet<Self>, value: &Self) -> bool {
                $module::remove(set, value)
            }

            fn reserve(set: &mut QSet<Self>, size: isize) {
                $module::reserve(set, size);
            }
        }
    };
}

#[cfg(all(feature = "request", cxxqt_qt_version_at_least_6_5))]
impl_qset_element!(
    QHttp1Configuration,
    qset_qhttp1configuration,
    "QSet_QHttp1Configuration"
);

#[cfg(feature = "ssl")]
impl_qset_element!(QOcspResponse, qset_qocspresponse, "QSet_QOcspResponse");

#[cfg(feature = "ssl")]
impl_qset_element!(
    QSslCertificate,
    qset_qsslcertificate,
    "QSet_QSslCertificate",
);

#[cfg(feature = "ssl")]
impl_qset_element!(
    QSslEllipticCurve,
    qset_qsslellipticcurve,
    "QSet_QSslEllipticCurve",
);

#[cfg(feature = "ssl")]
impl_qset_element!(QSslError, qset_qsslerror, "QSet_QSslError");
