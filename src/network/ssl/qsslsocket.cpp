#include "cxx-qt-io/qsslsocket.h"

namespace rust {
namespace cxxqtio1 {
::std::int64_t
qsslsocketSslLibraryBuildVersionNumber()
{
  return static_cast<::std::int64_t>(
    QSslSocket::sslLibraryBuildVersionNumber());
}

::std::int64_t
qsslsocketSslLibraryVersionNumber()
{
  return static_cast<::std::int64_t>(QSslSocket::sslLibraryVersionNumber());
}

}
}
