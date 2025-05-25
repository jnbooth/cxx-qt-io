#include "cxx-qt-io/qhttpheaders.h"

#include <cxx-qt-io/assertion_utils.h>
#include <cxx-qt-io/views.h>

assert_shared_pointer_type(QHttpHeaders);

namespace rust {
namespace cxxqtio1 {

::rust::Slice<const ::std::uint8_t>
qhttpheadersWellKnownHeaderName(QHttpHeaders::WellKnownHeader name)
{
  return qbytearrayviewAsSlice(QHttpHeaders::wellKnownHeaderName(name));
}

::rust::Str
qhttpheadersNameAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return qlatin1stringviewAsStr(headers.nameAt(static_cast<qsizetype>(i)));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers, QAnyStringView name)
{
  return qbytearrayviewAsSlice(headers.value(name));
}
::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers,
                  QHttpHeaders::WellKnownHeader name)
{
  return qbytearrayviewAsSlice(headers.value(name));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValueAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return qbytearrayviewAsSlice(headers.valueAt(static_cast<qsizetype>(i)));
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
