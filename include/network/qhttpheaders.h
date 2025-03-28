#pragma once

#include <QtNetwork/QHttpHeaders>

#include "rust/cxx.h"

using WellKnownHeader = QHttpHeaders::WellKnownHeader;
using QHttpHeadersEntry = ::std::pair<QByteArray, QByteArray>;
using QList_QHttpHeadersEntry = QList<QHttpHeadersEntry>;

namespace rust {
template<>
struct IsRelocatable<QHttpHeaders> : ::std::true_type
{};

template<>
struct IsRelocatable<QHttpHeadersEntry> : ::std::true_type
{};

namespace cxxqtio1 {
::rust::Str
qhttpheadersNameAt(const QHttpHeaders& headers, ::rust::isize i);

::rust::Slice<const ::std::uint8_t>
qhttpheadersValue(const QHttpHeaders& headers, QAnyStringView name);
::rust::Slice<const ::std::uint8_t>
qhttpheadersValue(const QHttpHeaders& headers, WellKnownHeader name);

::rust::Slice<const ::std::uint8_t>
qhttpheadersValueAt(const QHttpHeaders& headers, ::rust::isize i);
}

} // namespace rust
