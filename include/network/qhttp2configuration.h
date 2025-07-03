#pragma once

#include <QtNetwork/QHttp2Configuration>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QHttp2Configuration> : ::std::true_type
{};
}
