#pragma once

#include <QtNetwork/QHttpHeaders>

#include "rust/cxx.h"

using QHttpHeadersWellKnownHeader = QHttpHeaders::WellKnownHeader;

namespace rust {
template<>
struct IsRelocatable<QHttpHeaders> : ::std::true_type
{};

namespace cxxqtio1 {
::rust::Slice<const ::std::uint8_t>
qhttpheadersWellKnownHeaderName(QHttpHeaders::WellKnownHeader name);

::rust::Str
qhttpheadersNameAt(const QHttpHeaders& headers, ::rust::isize i);

::rust::Slice<const ::std::uint8_t>
qhttpheadersValue(const QHttpHeaders& headers, QAnyStringView name);
::rust::Slice<const ::std::uint8_t>
qhttpheadersValue(const QHttpHeaders& headers,
                  QHttpHeaders::WellKnownHeader name);

::rust::Slice<const ::std::uint8_t>
qhttpheadersValueAt(const QHttpHeaders& headers, ::rust::isize i);

bool
qhttpheadersEq(const QHttpHeaders& a, const QHttpHeaders& b);

}
}
