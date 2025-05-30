#pragma once

#include <QtNetwork/QHttpPart>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QHttpPart> : ::std::true_type
{};
}

