#pragma once

#include <QtNetwork/QNetworkCookie>

#include "rust/cxx.h"

using CookieRawForm = QNetworkCookie::RawForm;

#if (QT_VERSION >= QT_VERSION_CHECK(6, 1, 0))
using SameSitePolicy = QNetworkCookie::SameSite;
#endif

namespace rust {
template<>
struct IsRelocatable<QNetworkCookie> : ::std::true_type
{};

namespace cxxqtio1 {
QList<QNetworkCookie>
qnetworkcookieParseCookies(::rust::Slice<const ::std::uint8_t> cookieString);

}

} // namespace rust
