#include "cxx-qt-io/qhttpheaders.h"

#include <cxx-qt-io/views.h>
#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QHttpHeaders, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QHttpHeaders>::value);
static_assert(!::std::is_trivially_copy_constructible<QHttpHeaders>::value);

static_assert(!::std::is_trivially_destructible<QHttpHeaders>::value);

static_assert(QTypeInfo<QHttpHeaders>::isRelocatable);

namespace rust {
namespace xxqtio1 {

::rust::Slice<const ::std::uint8_t>
qhttpheadersWellKnownHeaderName(WellKnownHeader name)
{
  return convertView(QHttpHeaders::wellKnownHeaderName(name));
}

::rust::Str
qhttpheadersNameAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return convertView(headers.nameAt(static_cast<qsizetype>(i)));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers, QAnyStringView name)
{
  return convertView(headers.value(name));
}
::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers, WellKnownHeader name)
{
  return convertView(headers.value(name));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValueAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return convertView(headers.valueAt(static_cast<qsizetype>(i)));
}
}

} // namespace rust
