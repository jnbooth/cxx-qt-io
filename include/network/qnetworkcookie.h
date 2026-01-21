#pragma once

#include <QtNetwork/QNetworkCookie>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QNetworkCookie> : ::std::true_type
{};

namespace cxxqtio1 {
using QNetworkCookieRawForm = QNetworkCookie::RawForm;
using QNetworkCookieSameSite = QNetworkCookie::SameSite;

QList<QNetworkCookie>
qnetworkcookieParseCookies(::rust::Slice<const ::std::uint8_t> cookieString);

}

}
