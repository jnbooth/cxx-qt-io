#pragma once

#include <QtNetwork/QSslEllipticCurve>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslEllipticCurve> : ::std::true_type
{};

namespace cxxqtio1 {
QSslEllipticCurve (*qsslellipticcurveFromLongName)(const QString&) =
  QSslEllipticCurve::fromLongName;
QSslEllipticCurve (*qsslellipticcurveFromShortName)(const QString&) =
  QSslEllipticCurve::fromShortName;

}
} // namespace rust
