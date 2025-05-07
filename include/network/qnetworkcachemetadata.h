#pragma once

#include <QtNetwork/QNetworkCacheMetaData>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QNetworkCacheMetaData> : ::std::true_type
{};
}
