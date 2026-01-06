#include "cxx-qt-io/qhttpheaders.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QHttpHeaders);

namespace rust {
namespace cxxqtio1 {
namespace qhttpheaders {
inline ::rust::Slice<const ::std::uint8_t>
viewSlice(QByteArrayView view) noexcept
{
  return ::rust::Slice(reinterpret_cast<const ::std::uint8_t*>(view.data()),
                       view.size());
}

inline ::rust::Str
viewStr(QLatin1StringView view)
{
  return ::rust::Str(view.data(), view.size());
}
}

::rust::Slice<const ::std::uint8_t>
qhttpheadersWellKnownHeaderName(QHttpHeaders::WellKnownHeader name)
{
  return qhttpheaders::viewSlice(QHttpHeaders::wellKnownHeaderName(name));
}

::rust::Str
qhttpheadersNameAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return qhttpheaders::viewStr(headers.nameAt(static_cast<qsizetype>(i)));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers, QAnyStringView name)
{
  return qhttpheaders::viewSlice(headers.value(name));
}
::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers,
                  QHttpHeaders::WellKnownHeader name)
{
  return qhttpheaders::viewSlice(headers.value(name));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValueAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return qhttpheaders::viewSlice(headers.valueAt(static_cast<qsizetype>(i)));
}

bool
qhttpheadersEq(const QHttpHeaders& a, const QHttpHeaders& b)
{
  const qsizetype size = a.size();
  if (b.size() != size) {
    return false;
  }

  for (qsizetype i = 0; i < size; ++i) {
    if (a.nameAt(i) != b.nameAt(i) || a.valueAt(i) != b.valueAt(i)) {
      return false;
    }
  }

  return true;
}

}

}
