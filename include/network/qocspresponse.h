#pragma once

#include <QtNetwork/QOcspResponse>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QOcspResponse> : ::std::true_type
{};

} // namespace rust
