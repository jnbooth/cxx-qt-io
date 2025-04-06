#pragma once

#include <QtNetwork/QNetworkRequest>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QNetworkRequest> : ::std::true_type
{};
}

using QNetworkRequestAttribute = QNetworkRequest::Attribute;
using QNetworkRequestKnownHeaders = QNetworkRequest::KnownHeaders;
