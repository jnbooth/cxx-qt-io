#include "cxx-qt-io/qnetworkcookie.h"

#include <cxx-qt-io/assertion_utils.h>
#include <cxx-qt-io/views.h>

assert_shared_pointer_type(QNetworkCookie);

namespace rust {
namespace cxxqtio1 {

QList<QNetworkCookie>
qnetworkcookieParseCookies(::rust::Slice<const ::std::uint8_t> cookieString)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
  return QNetworkCookie::parseCookies(qbytearrayviewFromSlice(cookieString));
#else
  return QNetworkCookie::parseCookies(qbytearrayFromRawData(cookieString));
#endif
}

}
}
