#include "cxx-qt-io/qbuffer.h"

namespace rust {
namespace cxxqtio1 {
void
qbufferSetData(QBuffer& buffer, ::rust::Slice<const ::std::uint8_t> data)
{
  buffer.setData(reinterpret_cast<const char*>(data.data()), data.size());
}
}
} // namespace rust
