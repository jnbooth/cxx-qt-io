#pragma once

#include <QtNetwork/QSslSocket>

using QSslSocketPeerVerifyMode = QSslSocket::PeerVerifyMode;
using QSslSocketSslMode = QSslSocket::SslMode;

namespace rust {
namespace cxxqtio1 {
::std::int64_t
qsslsocketSslLibraryBuildVersionNumber();
inline QString (*qsslsocketSslLibraryBuildVersionString)() =
  QSslSocket::sslLibraryBuildVersionString;
::std::int64_t
qsslsocketSslLibraryVersionNumber();
inline QString (*qsslsocketSslLibraryVersionString)() =
  QSslSocket::sslLibraryVersionString;
inline bool (*qsslsocketSupportsSsl)() = QSslSocket::supportsSsl;

inline QString (*qsslsocketActiveBackend)() = QSslSocket::activeBackend;
inline QList<QString> (*qsslsocketAvailableBackends)() =
  QSslSocket::availableBackends;
inline QList<QSsl::ImplementedClass> (*qsslsocketImplementedClasses)(
  const QString&) = QSslSocket::implementedClasses;
inline bool (*qsslsocketIsClassImplemented)(QSsl::ImplementedClass,
                                            const QString&) =
  QSslSocket::isClassImplemented;
inline bool (*qsslsocketIsFeatureSupported)(QSsl::SupportedFeature,
                                            const QString&) =
  QSslSocket::isFeatureSupported;
inline bool (*qsslsocketIsProtocolSupported)(QSsl::SslProtocol,
                                             const QString&) =
  QSslSocket::isProtocolSupported;
inline bool (*qsslsocketSetActiveBackend)(const QString&) =
  QSslSocket::setActiveBackend;
inline QList<QSsl::SupportedFeature> (*qsslsocketSupportedFeatures)(
  const QString&) = QSslSocket::supportedFeatures;
inline QList<QSsl::SslProtocol> (*qsslsocketSupportedProtocols)(
  const QString&) = QSslSocket::supportedProtocols;
}
}
