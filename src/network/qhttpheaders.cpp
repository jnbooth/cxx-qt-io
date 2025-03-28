#include "cxx-qt-io/qhttpheaders.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QHttpHeaders, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QHttpHeaders>::value);
static_assert(!::std::is_trivially_copy_constructible<QHttpHeaders>::value);

static_assert(!::std::is_trivially_destructible<QHttpHeaders>::value);

static_assert(QTypeInfo<QHttpHeaders>::isRelocatable);

assert_alignment_and_size(QHttpHeadersEntry, {
  QByteArray name;
  QByteArray value;
});

static_assert(!::std::is_trivially_copy_assignable<QHttpHeadersEntry>::value);
static_assert(
  !::std::is_trivially_copy_constructible<QHttpHeadersEntry>::value);

static_assert(!::std::is_trivially_destructible<QHttpHeadersEntry>::value);

static_assert(QTypeInfo<QHttpHeadersEntry>::isRelocatable);

::rust::Slice<const ::std::uint8_t>
convertByteView(QByteArrayView view)
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
::rust::Str
qhttpheadersNameAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return convertStringView(headers.nameAt(static_cast<qsizetype>(i)));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers, QAnyStringView name)
{
  return convertByteView(headers.value(name));
}
::rust::Slice<const ::rust::u8>
qhttpheadersValue(const QHttpHeaders& headers, WellKnownHeader name)
{
  return convertByteView(headers.value(name));
}

::rust::Slice<const ::rust::u8>
qhttpheadersValueAt(const QHttpHeaders& headers, ::rust::isize i)
{
  return convertByteView(headers.valueAt(static_cast<qsizetype>(i)));
}
}

} // namespace rust
