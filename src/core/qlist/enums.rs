#![allow(clippy::wildcard_imports)]
use std::{mem, ptr};

use cxx::type_id;
use cxx_qt_lib::{QList, QListElement};

unsafe fn cast_enum_list<From, To>(list: &QList<From>) -> &QList<To>
where
    From: QListElement,
    To: QListElement,
{
    unsafe { &*(ptr::from_ref(list).cast()) }
}

unsafe fn cast_enum_list_mut<From, To>(list: &mut QList<From>) -> &mut QList<To>
where
    From: QListElement,
    To: QListElement,
{
    unsafe { &mut *(ptr::from_mut(list).cast()) }
}

macro_rules! impl_qlist_element {
    ($t:ty, $typeId:literal $(,)?) => {
        impl_qlist_element!($t, $typeId, i32);
    };
    ($t:ty, $typeId:literal, $r:ty) => {
        impl QListElement for $t {
            type TypeId = type_id!($typeId);

            fn append(list: &mut QList<Self>, value: Self) {
                <$r as QListElement>::append(unsafe { cast_enum_list_mut(list) }, value.repr);
            }
            fn append_clone(list: &mut QList<Self>, value: &Self) {
                <$r as QListElement>::append_clone(
                    unsafe { cast_enum_list_mut(list) },
                    &value.repr,
                );
            }
            fn clear(list: &mut QList<Self>) {
                <$r as QListElement>::clear(unsafe { cast_enum_list_mut(list) });
            }
            fn clone(list: &QList<Self>) -> QList<Self> {
                unsafe { mem::transmute(<$r as QListElement>::clone(cast_enum_list(list))) }
            }
            fn contains(list: &QList<Self>, value: &Self) -> bool {
                <$r as QListElement>::contains(unsafe { cast_enum_list(list) }, &value.repr)
            }
            fn default() -> QList<Self> {
                unsafe { mem::transmute(<$r as QListElement>::default()) }
            }
            fn drop(list: &mut QList<Self>) {
                <$r as QListElement>::drop(unsafe { cast_enum_list_mut(list) })
            }
            unsafe fn get_unchecked(list: &QList<Self>, pos: isize) -> &Self {
                unsafe {
                    &*ptr::from_ref(<$r as QListElement>::get_unchecked(
                        cast_enum_list(list),
                        pos,
                    ))
                    .cast()
                }
            }
            fn index_of(list: &QList<Self>, value: &Self) -> isize {
                <$r as QListElement>::index_of(unsafe { cast_enum_list(list) }, &value.repr)
            }
            fn insert(list: &mut QList<Self>, pos: isize, value: Self) {
                <$r as QListElement>::insert(unsafe { cast_enum_list_mut(list) }, pos, value.repr);
            }
            fn insert_clone(list: &mut QList<Self>, pos: isize, value: &Self) {
                <$r as QListElement>::insert_clone(
                    unsafe { cast_enum_list_mut(list) },
                    pos,
                    &value.repr,
                );
            }
            fn len(list: &QList<Self>) -> isize {
                <$r as QListElement>::len(unsafe { cast_enum_list(list) })
            }
            fn remove(list: &mut QList<Self>, pos: isize) {
                <$r as QListElement>::remove(unsafe { cast_enum_list_mut(list) }, pos);
            }
            fn reserve(list: &mut QList<Self>, size: isize) {
                <$r as QListElement>::reserve(unsafe { cast_enum_list_mut(list) }, size);
            }
        }
    };
}

impl_qlist_element!(
    crate::QCryptographicHashAlgorithm,
    "QList_QCryptographicHashAlgorithm"
);
impl_qlist_element!(crate::QFileDeviceFileError, "QList_QFileDeviceFileError");
impl_qlist_element!(
    crate::QFileDeviceFileHandleFlag,
    "QList_QFileDeviceFileHandleFlag"
);
impl_qlist_element!(crate::QFileDeviceFileTime, "QList_QFileDeviceFileTime");
impl_qlist_element!(
    crate::QFileDeviceMemoryMapFlag,
    "QList_QFileDeviceMemoryMapFlag"
);
impl_qlist_element!(crate::QFileDevicePermission, "QList_QFileDevicePermission");
impl_qlist_element!(crate::QIODeviceOpenModeFlag, "QList_QIODeviceOpenModeFlag");

#[cfg(feature = "qt_network")]
mod network {
    use super::*;
    impl_qlist_element!(
        crate::QAbstractSocketBindFlag,
        "QList_QAbstractSocketBindFlag"
    );
    impl_qlist_element!(
        crate::QAbstractSocketNetworkLayerProtocol,
        "QList_QAbstractSocketNetworkLayerProtocol"
    );
    impl_qlist_element!(
        crate::QAbstractSocketPauseMode,
        "QList_QAbstractSocketPauseMode"
    );
    impl_qlist_element!(
        crate::QAbstractSocketSocketError,
        "QList_QAbstractSocketSocketError"
    );
    impl_qlist_element!(
        crate::QAbstractSocketSocketOption,
        "QList_QAbstractSocketSocketOption"
    );
    impl_qlist_element!(
        crate::QAbstractSocketSocketState,
        "QList_QAbstractSocketSocketState"
    );
    impl_qlist_element!(
        crate::QAbstractSocketSocketType,
        "QList_QAbstractSocketSocketType"
    );

    impl_qlist_element!(
        crate::QHostAddressConversionModeFlag,
        "QList_QHostAddressConversionModeFlag"
    );
    impl_qlist_element!(
        crate::QHostAddressSpecialAddress,
        "QList_QHostAddressSpecialAddress"
    );

    #[cfg(cxxqt_qt_version_at_least_6_7)]
    impl_qlist_element!(
        crate::QHttpHeadersWellKnownHeader,
        "QList_QHttpHeadersWellKnownHeader"
    );

    impl_qlist_element!(
        crate::QNetworkAddressEntryDnsEligibilityStatus,
        "QList_QNetworkAddressEntryDnsEligibilityStatus",
        i8
    );

    impl_qlist_element!(crate::QNetworkCookieRawForm, "QList_QNetworkCookieRawForm");
    #[cfg(cxxqt_qt_version_at_least_6_1)]
    impl_qlist_element!(
        crate::QNetworkCookieSameSite,
        "QList_QNetworkCookieSameSite"
    );

    impl_qlist_element!(
        crate::QNetworkInterfaceInterfaceFlag,
        "QList_QNetworkInterfaceInterfaceFlag"
    );
    impl_qlist_element!(
        crate::QNetworkInterfaceInterfaceType,
        "QList_QNetworkInterfaceInterfaceType"
    );

    impl_qlist_element!(
        crate::QNetworkProxyCapability,
        "QList_QNetworkProxyCapability"
    );
    impl_qlist_element!(
        crate::QNetworkProxyProxyType,
        "QList_QNetworkProxyProxyType"
    );

    impl_qlist_element!(
        crate::QNetworkRequestKnownHeaders,
        "QList_QNetworkRequestKnownHeaders"
    );
}

#[cfg(feature = "ssl")]
mod ssl {
    use super::*;
    impl_qlist_element!(
        crate::QOcspCertificateStatus,
        "QList_QOcspCertificateStatus"
    );
    impl_qlist_element!(crate::QOcspRevocationReason, "QList_QOcspRevocationReason");

    impl_qlist_element!(
        crate::QSslAlternativeNameEntryType,
        "QList_QSslAlternativeNameEntryType"
    );
    impl_qlist_element!(crate::QSslEncodingFormat, "QList_QSslEncodingFormat");
    impl_qlist_element!(crate::QSslKeyAlgorithm, "QList_QSslKeyAlgorithm");
    impl_qlist_element!(crate::QSslKeyType, "QList_QSslKeyType");
    impl_qlist_element!(crate::QSslSslOption, "QList_QSslSslOption");
    impl_qlist_element!(crate::QSslSslProtocol, "QList_QSslSslProtocol");

    impl_qlist_element!(
        crate::QSslCertificatePatternSyntax,
        "QList_QSslCertificatePatternSyntax"
    );
    impl_qlist_element!(
        crate::QSslCertificateSubjectInfo,
        "QList_QSslCertificateSubjectInfo"
    );

    impl_qlist_element!(
        crate::QSslConfigurationNextProtocolNegotiationStatus,
        "QList_QSslConfigurationNextProtocolNegotiationStatus"
    );

    impl_qlist_element!(crate::QSslErrorSslError, "QList_QSslErrorSslError");

    impl_qlist_element!(
        crate::QSslSocketPeerVerifyMode,
        "QList_QSslSocketPeerVerifyMode"
    );
    impl_qlist_element!(crate::QSslSocketSslMode, "QList_QSslSocketSslMode");

    #[cfg(cxxqt_qt_version_at_least_6_0)]
    impl_qlist_element!(crate::QSslAlertLevel, "QList_QSslAlertLevel");
    #[cfg(cxxqt_qt_version_at_least_6_0)]
    impl_qlist_element!(crate::QSslAlertType, "QList_QSslAlertType");
    #[cfg(cxxqt_qt_version_at_least_6_1)]
    impl_qlist_element!(crate::QSslImplementedClass, "QList_QSslImplementedClass");
    #[cfg(cxxqt_qt_version_at_least_6_1)]
    impl_qlist_element!(crate::QSslSupportedFeature, "QList_QSslSupportedFeature");
}
