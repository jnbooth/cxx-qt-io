#pragma once

#include <QtCore/QObject>

namespace rust {
namespace cxxqtio1 {
const char*
qobjectClassName(const QObject& obj);

bool
qobjectThreadEq(const QObject& lhs, const QObject& rhs);

}
}
