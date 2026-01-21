#pragma once

#include <QtCore/QDeadlineTimer>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QDeadlineTimer> : ::std::true_type
{};

namespace cxxqtio1 {
using QDeadlineTimerForeverConstant = QDeadlineTimer::ForeverConstant;
}
}
