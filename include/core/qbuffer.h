#pragma once

#include <QtCore/QBuffer>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtio1 {
void
qbufferSetData(QBuffer& buffer, ::rust::Slice<const ::std::uint8_t> data);
}
} // namespace rust
