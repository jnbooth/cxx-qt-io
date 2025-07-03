#pragma once

#include <QtNetwork/QHttp1Configuration>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QHttp1Configuration> : ::std::true_type
{};
}
