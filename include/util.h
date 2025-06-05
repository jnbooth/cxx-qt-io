#pragma once

#include <QtCore/QObject>

namespace rust {
namespace cxxqtio1 {
inline void
qobjectDelete(QObject* object)
{
  delete object;
}

inline bool
qobjectThreadEq(const QObject& lhs, const QObject& rhs)
{
  return lhs.thread() == rhs.thread();
}

}
}
