#pragma once

#include <QtCore/QByteArrayView>
#include <QtCore/QLatin1StringView>

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

inline ::rust::Str
qlatin1stringviewAsStr(QLatin1StringView view)
{
  return ::rust::Str(view.data(), view.size());
}

}
}
