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
}
}
