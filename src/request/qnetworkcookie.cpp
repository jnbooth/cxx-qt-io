#include "cxx-qt-io/qnetworkcookie.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QNetworkCookie);

namespace rust {
namespace cxxqtio1 {

QList<QNetworkCookie>
qnetworkcookieParseCookies(::rust::Slice<const ::std::uint8_t> cookieString)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
  using Bytes = QByteArrayView;
#else
  using Bytes = QByteArray;
#endif
  return QNetworkCookie::parseCookies(Bytes(
    reinterpret_cast<const char*>(cookieString.data()), cookieString.size()));
}

}
}
