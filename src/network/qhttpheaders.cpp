#include "cxx-qt-io/qhttpheaders.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QHttpHeaders, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QHttpHeaders>::value);
static_assert(!::std::is_trivially_copy_constructible<QHttpHeaders>::value);

static_assert(!::std::is_trivially_destructible<QHttpHeaders>::value);

static_assert(QTypeInfo<QHttpHeaders>::isRelocatable);

::rust::Slice<const ::std::uint8_t>
convertBytesView(QByteArrayView view)
{
  return ::rust::Slice(reinterpret_cast<const ::std::uint8_t*>(view.data()),
                       view.size());
}

::rust::Str
convertStringView(QLatin1StringView view)
{
  return ::rust::Str(view.data(), view.size());
}

namespace rust {
namespace xxqtio1 {

::rust::Slice<const ::std::uint8_t>
qhttpheadersWellKnownHeaderName(WellKnownHeader name)
{
  return convertBytesView(QHttpHeaders::wellKnownHeaderName(name));
}

::rust::Str
qhttpheadersNameAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return convertStringView(headers.nameAt(static_cast<qsizetype>(i)));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers, QAnyStringView name)
{
  return convertBytesView(headers.value(name));
}
::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers, WellKnownHeader name)
{
  return convertBytesView(headers.value(name));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValueAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return convertBytesView(headers.valueAt(static_cast<qsizetype>(i)));
}
}

} // namespace rust
