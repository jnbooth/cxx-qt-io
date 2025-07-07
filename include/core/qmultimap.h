#pragma once

#include <QtCore/QMultiMap>

namespace rust {
namespace cxxqtio1 {

template<typename K, typename V>
class QMultiMapEntryIter
{
public:
  QMultiMapEntryIter(const QMultiMap<K, V>& map)
    : iter(--map.constKeyValueBegin())
    , end(map.constKeyValueEnd())
  {
  }

  QMultiMapEntryIter(QMultiMapEntryIter<K, V>&& other) = default;

  bool advance() { return ++iter != end; }

  const K& key() const { return iter->first; }
  const V& value() const { return iter->second; }

private:
  typename QMultiMap<K, V>::const_key_value_iterator iter;
  typename QMultiMap<K, V>::const_key_value_iterator end;
};

template<typename K, typename V>
class QMultiMapKeyIter
{
public:
  QMultiMapKeyIter(const QMultiMap<K, V>& map)
    : iter(--map.keyBegin())
    , end(map.keyEnd())
  {
  }

  QMultiMapKeyIter(QMultiMapKeyIter<K, V>&& other) = default;

  bool advance() { return ++iter != end; }

  const K& key() const { return *iter; }

private:
  typename QMultiMap<K, V>::key_iterator iter;
  typename QMultiMap<K, V>::key_iterator end;
};

template<typename K, typename V>
class QMultiMapValueIter
{
public:
  QMultiMapValueIter(const QMultiMap<K, V>& map, const K& key)
  {
    auto [range_start, range_end] = map.equal_range(key);
    iter = --range_start;
    end = range_end;
  }

  QMultiMapValueIter(QMultiMapValueIter<K, V>&& other) = default;

  bool advance() { return ++iter != end; }

  const V& value() const { return *iter; }

private:
  typename QMultiMap<K, V>::const_iterator iter;
  typename QMultiMap<K, V>::const_iterator end;
};

template<typename K, typename V>
QMultiMapValueIter<K, V>
qmultimapFind(const QMultiMap<K, V>& map, const K& key)
{
  return QMultiMapValueIter(map, key);
}

template<typename K, typename V>
QMultiMapKeyIter<K, V>
qmultimapKeys(const QMultiMap<K, V>& map)
{
  return QMultiMapKeyIter(map);
}

template<typename K, typename V>
QMultiMapEntryIter<K, V>
qmultimapIter(const QMultiMap<K, V>& map)
{
  return QMultiMapEntryIter(map);
}

}
}
