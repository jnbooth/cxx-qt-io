#pragma once

#include <QtCore/QDeadlineTimer>

#include "rust/cxx.h"

using QDeadlineTimerForeverConstant = QDeadlineTimer::ForeverConstant;

namespace rust {
template<>
struct IsRelocatable<QDeadlineTimer> : ::std::true_type
{};
}
