#pragma once

#include <QtCore/QByteArray>
#include <QtCore/QByteArrayView>
#if (QT_VERSION >= QT_VERSION_CHECK(6, 4, 0))
#include <QtCore/QLatin1StringView>
#endif

#include "rust/cxx.h"

namespace rust {
namespace cxxqtio1 {
inline ::rust::Slice<const ::std::uint8_t>
qbytearrayviewAsSlice(QByteArrayView view)
{
  return ::rust::Slice(reinterpret_cast<const ::std::uint8_t*>(view.data()),
                       view.size());
}

inline QByteArrayView
qbytearrayviewFromSlice(::rust::Slice<const ::std::uint8_t> slice)
{
  return QByteArrayView(reinterpret_cast<const char*>(slice.data()),
                        slice.size());
}

inline QByteArray
qbytearrayFromRawData(::rust::Slice<const ::std::uint8_t> slice)
{
  return QByteArray(reinterpret_cast<const char*>(slice.data()), slice.size());
}

#if (QT_VERSION >= QT_VERSION_CHECK(6, 4, 0))
inline ::rust::Str
qlatin1stringviewAsStr(QLatin1StringView view)
{
  return ::rust::Str(view.data(), view.size());
}
#endif
}
}
