#pragma once

#include <QtCore/QDeadlineTimer>

#include "rust/cxx.h"

using QDeadlineTimerForeverConstant = QDeadlineTimer::ForeverConstant;

namespace rust {
template<>
struct IsRelocatable<QDeadlineTimer> : ::std::true_type
{};

namespace cxxqtio1 {
QDeadlineTimer (*qdeadlinetimerAddNSecs)(QDeadlineTimer,
                                         qint64) = QDeadlineTimer::addNSecs;

QDeadlineTimer (*qdeadlinetimerCurrent)(Qt::TimerType) =
  QDeadlineTimer::current;

}
}
