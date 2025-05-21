#pragma once

#include <QtCore/QList>
#include <QtCore/QPair>

namespace rust {
namespace cxxqtio1 {
template<typename T>
struct FirstChecker
{
  const T& first;
};

template<typename T1, typename T2>
bool
operator==(const QPair<T1, T2>& lhs, const FirstChecker<T1>& rhs)
{
  return lhs.first == rhs.first;
}

template<typename P, typename T1, typename T2>
void
qpairlistAppend(QList<P>& list, T1 first, T2 second)
{
  list.emplaceBack(first, second);
}

template<typename P, typename T1, typename T2>
void
qpairlistInsert(QList<P>& list, qsizetype i, T1 first, T2 second)
{
  list.emplace(i, first, second);
}

template<typename T1, typename T2>
bool
qpairlistContains(const QList<QPair<T1, T2>>& list, const T1& key)
{
  return list.contains(FirstChecker<T1>{ key });
}

template<typename T1, typename T2>
qsizetype
qpairlistIndexOf(const QList<QPair<T1, T2>>& list, const T1& key)
{
  return list.indexOf(FirstChecker<T1>{ key });
}

}
}
