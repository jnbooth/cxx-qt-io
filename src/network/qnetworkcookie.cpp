#include "cxx-qt-io/qnetworkcookie.h"

#include <cxx-qt-io/views.h>
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QNetworkCookie, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QNetworkCookie>::value);
static_assert(!::std::is_trivially_copy_constructible<QNetworkCookie>::value);

static_assert(!::std::is_trivially_destructible<QNetworkCookie>::value);

static_assert(QTypeInfo<QNetworkCookie>::isRelocatable);

namespace rust {
namespace cxxqtio1 {

QList<QNetworkCookie>
qnetworkcookieParseCookies(const QByteArray& cookieString)
{
  return QNetworkCookie::parseCookies(cookieString);
}

}
}
