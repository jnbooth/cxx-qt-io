#pragma once

#include <QtCore/QByteArrayView>
#include <QtCore/QLatin1StringView>

#include "rust/cxx.h"

inline ::rust::Slice<const ::std::uint8_t>
convertView(QByteArrayView view)
{
  return ::rust::Slice(reinterpret_cast<const ::std::uint8_t*>(view.data()),
                       view.size());
}

inline QByteArrayView
convertView(::rust::Slice<const ::std::uint8_t> slice)
{
  return QByteArrayView(reinterpret_cast<const char*>(slice.data()),
                        slice.size());
}

inline ::rust::Str
convertView(QLatin1StringView view)
{
  return ::rust::Str(view.data(), view.size());
}
