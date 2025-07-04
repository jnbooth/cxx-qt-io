#pragma once

#include <QtNetwork/QSslConfiguration>

#include "rust/cxx.h"

using QSslConfigurationNextProtocolNegotiationStatus =
  QSslConfiguration::NextProtocolNegotiationStatus;

namespace rust {
template<>
struct IsRelocatable<QSslConfiguration> : ::std::true_type
{};

namespace cxxqtio1 {
inline QSslConfiguration (*qsslconfigurationDefaultConfiguration)() =
  QSslConfiguration::defaultConfiguration;

inline QSslConfiguration (*qsslconfigurationDefaultDtlsConfiguration)() =
  QSslConfiguration::defaultDtlsConfiguration;

inline void (*qsslconfigurationSetDefaultConfiguration)(
  const QSslConfiguration&) = QSslConfiguration::setDefaultConfiguration;

inline void (*qsslconfigurationSetDefaultDtlsConfiguration)(
  const QSslConfiguration&) = QSslConfiguration::setDefaultDtlsConfiguration;

inline QList<QSslCipher> (*qsslconfigurationSupportedCiphers)() =
  QSslConfiguration::supportedCiphers;

inline QList<QSslEllipticCurve> (*qsslconfigurationSupportedEllipticCurves)() =
  QSslConfiguration::supportedEllipticCurves;

inline QList<QSslCertificate> (*qsslconfigurationSystemCaCertificates)() =
  QSslConfiguration::systemCaCertificates;

::rust::String
qsslconfigurationALPNProtocolHTTP2();

::rust::String
qsslconfigurationNextProtocolHttp1_1();

}
}
