#pragma once

#include <QtCore/QByteArray>

#include <cxx-qt-lib/qlist_private.h>

namespace rust {
namespace cxxqtio1 {
namespace qlist {
template<typename T>
void
qlistClear(QList<T>& list)
{
  list.clear();
}

template<typename T>
bool
qlistContains(const QList<T>& list, const T& item)
{
  return list.contains(item);
}
}

}

} // namespace rust
