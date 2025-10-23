#pragma once

#include <cxx-qt-lib/core/qhash/qhash_private.h>

namespace rust {
namespace cxxqtio1 {
namespace qhash {
template<typename K, typename V>
void
qhashClear(QHash<K, V>& hash)
{
  hash.clear();
}

template<typename K, typename V>
bool
qhashContains(const QHash<K, V>& hash, const K& key)
{
  return hash.contains(key);
}
}

}

}
