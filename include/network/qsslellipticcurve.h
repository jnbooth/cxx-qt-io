#pragma once

#include <QtNetwork/QSslEllipticCurve>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslEllipticCurve> : ::std::true_type
{};

namespace cxxqtio1 {
inline QSslEllipticCurve (*qsslellipticcurveFromLongName)(const QString&) =
  QSslEllipticCurve::fromLongName;
inline QSslEllipticCurve (*qsslellipticcurveFromShortName)(const QString&) =
  QSslEllipticCurve::fromShortName;
}
}
