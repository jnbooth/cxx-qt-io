#include "cxx-qt-io/qbytearray.h"

namespace rust {
namespace cxxqtio1 {

QByteArray
qbytearrayToBase64(const QByteArray& bytes)
{
  return bytes.toBase64();
}
}
}
