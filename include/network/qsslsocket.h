#pragma once

#include <QtNetwork/QSslSocket>

namespace rust {
namespace cxxqtio1 {
using QSslSocketPeerVerifyMode = QSslSocket::PeerVerifyMode;
using QSslSocketSslMode = QSslSocket::SslMode;

::std::int64_t
qsslsocketSslLibraryBuildVersionNumber();
inline QString (*qsslsocketSslLibraryBuildVersionString)() =
  QSslSocket::sslLibraryBuildVersionString;
::std::int64_t
qsslsocketSslLibraryVersionNumber();
inline QString (*qsslsocketSslLibraryVersionString)() =
  QSslSocket::sslLibraryVersionString;
}
}
