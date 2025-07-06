#include "cxx-qt-io/qobject.h"

namespace rust {
namespace cxxqtio1 {

const char*
qobjectClassName(const QObject& obj)
{
  return obj.metaObject()->className();
}

bool
qobjectThreadEq(const QObject& lhs, const QObject& rhs)
{
  return lhs.thread() == rhs.thread();
}

}
}
