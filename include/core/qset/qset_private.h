#pragma once

#include <cxx-qt-lib/core/qset/qset_private.h>

namespace rust {
namespace cxxqtio1 {
namespace qset {
template<typename T>
void
qsetClear(QSet<T>& set)
{
  set.clear();
}

template<typename T>
bool
qsetContains(const QSet<T>& set, const T& item)
{
  return set.contains(item);
}

template<typename T>
bool
qsetRemove(QSet<T>& set, const T& item)
{
  return set.remove(item);
}

}
}
}
