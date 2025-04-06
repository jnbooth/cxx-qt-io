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
QSslConfiguration (*qsslconfigurationDefaultConfiguration)() =
  QSslConfiguration::defaultConfiguration;

QSslConfiguration (*qsslconfigurationDefaultDtlsConfiguration)() =
  QSslConfiguration::defaultDtlsConfiguration;

void (*qsslconfigurationSetDefaultConfiguration)(const QSslConfiguration&) =
  QSslConfiguration::setDefaultConfiguration;

void (*qsslconfigurationSetDefaultDtlsConfiguration)(const QSslConfiguration&) =
  QSslConfiguration::setDefaultDtlsConfiguration;

QList<QSslCipher> (*qsslconfigurationSupportedCiphers)() =
  QSslConfiguration::supportedCiphers;

QList<QSslEllipticCurve> (*qsslconfigurationSupportedEllipticCurves)() =
  QSslConfiguration::supportedEllipticCurves;

QList<QSslCertificate> (*qsslconfigurationSystemCaCertificates)() =
  QSslConfiguration::systemCaCertificates;

}
}
