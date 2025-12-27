#pragma once

#include <QtNetwork/QSslEllipticCurve>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslEllipticCurve> : ::std::true_type
{};
}
