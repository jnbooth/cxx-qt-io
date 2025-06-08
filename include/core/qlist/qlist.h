#pragma once

#include <QtCore/QCryptographicHash>
#include <QtCore/QFileDevice>

#include <cxx-qt-io/definitions.h>
#include <cxx-qt-lib/qlist.h>

#include "qlist_qdeadlinetimer.h"
#include "qlist_qpair_qbytearray_qbytearray.h"

#ifdef CXX_QT_IO_NET_FEATURE
#include "qlist_qhostaddress.h"
#include "qlist_qnetworkaddressentry.h"
#include "qlist_qnetworkproxy.h"

#include "qlist_qnetworkdatagram.h"
#include "qlist_qnetworkinterface.h"

#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
#include "qlist_qhttpheaders.h"
#endif
#endif

#ifdef CXX_QT_IO_REQUEST_FEATURE
#include "qlist_qhstspolicy.h"
#include "qlist_qhttp2configuration.h"
#include "qlist_qhttppart.h"
#include "qlist_qnetworkcachemetadata.h"
#include "qlist_qnetworkcookie.h"
#include "qlist_qnetworkrequest.h"

#if (QT_VERSION >= QT_VERSION_CHECK(6, 5, 0))
#include "qlist_qhttp1configuration.h"
#endif
#endif

#ifdef CXX_QT_IO_SSL_FEATURE
#include "qlist_qsslcertificate.h"
#include "qlist_qsslcipher.h"
#include "qlist_qsslconfiguration.h"
#include "qlist_qssldiffiehellmanparameters.h"
#include "qlist_qsslellipticcurve.h"
#include "qlist_qsslerror.h"
#include "qlist_qsslkey.h"
#include "qlist_qsslpresharedkeyauthenticator.h"

#include "qlist_qdtlsgeneratorparameters.h"
#include "qlist_qocspresponse.h"
#include "qlist_qsslcertificateextension.h"
#endif

using QList_QCryptographicHashAlgorithm = QList<QCryptographicHash::Algorithm>;

using QList_QFileDeviceFileError = QList<QFileDevice::FileError>;
using QList_QFileDeviceFileHandleFlag = QList<QFileDevice::FileHandleFlag>;
using QList_QFileDeviceFileTime = QList<QFileDevice::FileTime>;
using QList_QFileDeviceMemoryMapFlag = QList<QFileDevice::MemoryMapFlag>;
using QList_QFileDevicePermission = QList<QFileDevice::Permission>;

using QList_QIODeviceOpenModeFlag = QList<QIODevice::OpenModeFlag>;

#ifdef CXX_QT_IO_NET_FEATURE
using QList_QAbstractSocketBindFlag = QList<QAbstractSocket::BindFlag>;
using QList_QAbstractSocketBindMode = QList<QAbstractSocket::BindMode>;
using QList_QAbstractSocketNetworkLayerProtocol =
  QList<QAbstractSocket::NetworkLayerProtocol>;
using QList_QAbstractSocketPauseMode = QList<QAbstractSocket::PauseMode>;
using QList_QAbstractSocketSocketError = QList<QAbstractSocket::SocketError>;
using QList_QAbstractSocketSocketOption = QList<QAbstractSocket::SocketOption>;
using QList_QAbstractSocketSocketState = QList<QAbstractSocket::SocketState>;
using QList_QAbstractSocketSocketType = QList<QAbstractSocket::SocketType>;

using QList_QHostAddressConversionModeFlag =
  QList<QHostAddress::ConversionModeFlag>;
using QList_QHostAddressSpecialAddress = QList<QHostAddress::SpecialAddress>;
using QList_QNetworkAddressEntryDnsEligibilityStatus =
  QList<QNetworkAddressEntry::DnsEligibilityStatus>;

using QList_QNetworkInterfaceInterfaceFlag =
  QList<QNetworkInterface::InterfaceFlag>;
using QList_QNeteworkInterfaceInterfaceType =
  QList<QNetworkInterface::InterfaceType>;

using QList_QNetworkProxyCapability = QList<QNetworkProxy::Capability>;
using QList_QNetworkProxyProxyType = QList<QNetworkProxy::ProxyType>;

using QList_QNetworkRequestKnownHeaders = QList<QNetworkRequest::KnownHeaders>;

#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
using QList_QHttpHeadersWellKnownHeader = QList<QHttpHeaders::WellKnownHeader>;
#endif
#endif

#ifdef CXX_QT_IO_REQUEST_FEATURE
using QList_QNetworkCookieRawForm = QList<QNetworkCookieRawForm>;

#if (QT_VERSION >= QT_VERSION_CHECK(6, 1, 0))
using QList_QNetworkCookieSameSite = QList<QNetworkCookie::SameSite>;
#endif
#endif

#ifdef CXX_QT_IO_SSL_FEATURE
using QList_QOcspCertificateStatus = QList<QOcspCertificateStatus>;
using QList_QOcspRevocationReason = QList<QOcspRevocationReason>;

using QList_QSslAlternativeNameEntryType =
  QList<QSsl::AlternativeNameEntryType>;
using QList_QSslEncodingFormat = QList<QSsl::EncodingFormat>;
using QList_QSslKeyAlgorithm = QList<QSsl::KeyAlgorithm>;
using QList_QSslKeyType = QList<QSsl::KeyType>;
using QList_QSslSslOptions = QList<QSsl::SslOptions>;
using QList_QSslSslProtocol = QList<QSsl::SslProtocol>;

using QList_QSslCertificatePatternSyntax =
  QList<QSslCertificate::PatternSyntax>;
using QList_QSslCertificateSubjectInfo = QList<QSslCertificate::SubjectInfo>;

using QList_QSslConfigurationNextProtocolNegotiationStatus =
  QList<QSslConfiguration::NextProtocolNegotiationStatus>;

using QList_QSslErrorSslError = QList<QSslError::SslError>;

using QList_QSslSocketPeerVerifyMode = QList<QSslSocket::PeerVerifyMode>;
using QList_QSslSocketSslMode = QList<QSslSocket::SslMode>;
using QList_QSslAlertLevel = QList<QSsl::AlertLevel>;
using QList_QSslAlertType = QList<QSsl::AlertType>;

#if (QT_VERSION >= QT_VERSION_CHECK(6, 1, 0))
using QList_QSslImplementedClass = QList<QSsl::ImplementedClass>;
using QList_QSslSupportedFeature = QList<QSsl::SupportedFeature>;
#endif
#endif
