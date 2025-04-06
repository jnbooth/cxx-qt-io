#pragma once

#include <QtNetwork/QSslSocket>

using QSslSocketPeerVerifyMode = QSslSocket::PeerVerifyMode;
using QSslSocketSslMode = QSslSocket::SslMode;

namespace rust {
namespace cxxqtio1 {
::std::int64_t
qsslsocketSslLibraryBuildVersionNumber();
QString (*qsslsocketSslLibraryBuildVersionString)() =
  QSslSocket::sslLibraryBuildVersionString;
::std::int64_t
qsslsocketSslLibraryVersionNumber();
QString (*qsslsocketSslLibraryVersionString)() =
  QSslSocket::sslLibraryVersionString;
bool (*qsslsocketSupportsSsl)() = QSslSocket::supportsSsl;

#if (QT_VERSION >= QT_VERSION_CHECK(6, 1, 0))
QString (*qsslsocketActiveBackend)() = QSslSocket::activeBackend;
QList<QString> (*qsslsocketAvailableBackends)() = QSslSocket::availableBackends;
QList<QSsl::ImplementedClass> (*qsslsocketImplementedClasses)(const QString&) =
  QSslSocket::implementedClasses;
bool (*qsslsocketIsClassImplemented)(QSsl::ImplementedClass, const QString&) =
  QSslSocket::isClassImplemented;
bool (*qsslsocketIsFeatureSupported)(QSsl::SupportedFeature, const QString&) =
  QSslSocket::isFeatureSupported;
bool (*qsslsocketIsProtocolSupported)(QSsl::SslProtocol, const QString&) =
  QSslSocket::isProtocolSupported;
bool (*qsslsocketSetActiveBackend)(const QString&) =
  QSslSocket::setActiveBackend;
QList<QSsl::SupportedFeature> (*qsslsocketSupportedFeatures)(const QString&) =
  QSslSocket::supportedFeatures;
QList<QSsl::SslProtocol> (*qsslsocketSupportedProtocols)(const QString&) =
  QSslSocket::supportedProtocols;
#endif

}
}
