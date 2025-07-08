#pragma once

#include <QtNetwork/QNetworkCookie>

#include "rust/cxx.h"

using QNetworkCookieRawForm = QNetworkCookie::RawForm;
using QNetworkCookieSameSite = QNetworkCookie::SameSite;

namespace rust {
template<>
struct IsRelocatable<QNetworkCookie> : ::std::true_type
{};

namespace cxxqtio1 {
QList<QNetworkCookie>
qnetworkcookieParseCookies(::rust::Slice<const ::std::uint8_t> cookieString);

}

}
