mod enums;

use cxx::type_id;
use cxx_qt_lib::{QList, QListElement};

macro_rules! impl_qlist_element {
    ( $typeName:ty, $module:ident, $typeId:literal $(,)? ) => {
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
    crate::QPair<cxx_qt_lib::QByteArray, cxx_qt_lib::QByteArray>,
    qlist_qpair_qbytearray_qbytearray,
    "QList_QPair_QByteArray_QByteArray",
);

impl_qlist_element!(
    crate::QDeadlineTimer,
    qlist_qdeadlinetimer,
    "QList_QDeadlineTimer"
);

#[cfg(all(feature = "net", cxxqt_qt_version_at_least_6_7))]
impl_qlist_element!(
    crate::QHttpHeaders,
    qlist_qhttpheaders,
    "QList_QHttpHeaders",
);

#[cfg(feature = "net")]
impl_qlist_element!(
    crate::QHostAddress,
    qlist_qhostaddress,
    "QList_QHostAddress",
);

#[cfg(feature = "net")]
impl_qlist_element!(
    crate::QNetworkAddressEntry,
    qlist_qnetworkaddressentry,
    "QList_QNetworkAddressEntry",
);

#[cfg(feature = "net")]
impl_qlist_element!(
    crate::QNetworkDatagram,
    qlist_qnetworkdatagram,
    "QList_QNetworkDatagram",
);

#[cfg(feature = "net")]
impl_qlist_element!(
    crate::QNetworkInterface,
    qlist_qnetworkinterface,
    "QList_QNetworkInterface",
);

#[cfg(feature = "net")]
impl_qlist_element!(
    crate::QNetworkProxy,
    qlist_qnetworkproxy,
    "QList_QNetworkProxy",
);

#[cfg(feature = "request")]
impl_qlist_element!(crate::QHstsPolicy, qlist_qhstspolicy, "QList_QHstsPolicy");

#[cfg(all(feature = "request", cxxqt_qt_version_at_least_6_5))]
impl_qlist_element!(
    crate::QHttp1Configuration,
    qlist_qhttp1configuration,
    "QList_QHttp1Configuration",
);

#[cfg(feature = "request")]
impl_qlist_element!(
    crate::QHttp2Configuration,
    qlist_qhttp2configuration,
    "QList_QHttp2Configuration",
);

#[cfg(feature = "request")]
impl_qlist_element!(crate::QHttpPart, qlist_qhttppart, "QList_QHttpPart");

#[cfg(feature = "request")]
impl_qlist_element!(
    crate::QNetworkCookie,
    qlist_qnetworkcookie,
    "QList_QNetworkCookie",
);

#[cfg(feature = "request")]
impl_qlist_element!(
    crate::QNetworkRequest,
    qlist_qnetworkrequest,
    "QList_QNetworkRequest",
);

#[cfg(feature = "ssl")]
impl_qlist_element!(
    crate::QOcspResponse,
    qlist_qocspresponse,
    "QList_QOcspResponse",
);

#[cfg(feature = "ssl")]
impl_qlist_element!(
    crate::QDtlsGeneratorParameters,
    qlist_qdtlsgeneratorparameters,
    "QList_QDtlsGeneratorParameters",
);

#[cfg(feature = "ssl")]
impl_qlist_element!(
    crate::QSslCertificate,
    qlist_qsslcertificate,
    "QList_QSslCertificate",
);

#[cfg(feature = "ssl")]
impl_qlist_element!(
    crate::QSslCertificateExtension,
    qlist_qsslcertificateextension,
    "QList_QSslCertificateExtension",
);

#[cfg(feature = "ssl")]
impl_qlist_element!(crate::QSslCipher, qlist_qsslcipher, "QList_QSslCipher",);

#[cfg(feature = "ssl")]
impl_qlist_element!(
    crate::QSslConfiguration,
    qlist_qsslconfiguration,
    "QList_QSslConfiguration",
);

#[cfg(feature = "ssl")]
impl_qlist_element!(
    crate::QSslDiffieHellmanParameters,
    qlist_qssldiffiehellmanparameters,
    "QList_QSslDiffieHellmanParameters",
);

#[cfg(feature = "ssl")]
impl_qlist_element!(
    crate::QSslEllipticCurve,
    qlist_qsslellipticcurve,
    "QList_QSslEllipticCurve",
);

#[cfg(feature = "ssl")]
impl_qlist_element!(crate::QSslError, qlist_qsslerror, "QList_QSslError");

#[cfg(feature = "ssl")]
impl_qlist_element!(crate::QSslKey, qlist_qsslkey, "QList_QSslKey");

#[cfg(feature = "ssl")]
impl_qlist_element!(
    crate::QSslPreSharedKeyAuthenticator,
    qlist_qsslpresharedkeyauthenticator,
    "QList_QSslPreSharedKeyAuthenticator"
);
