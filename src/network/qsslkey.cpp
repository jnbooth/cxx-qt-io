#include "cxx-qt-io/qsslkey.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QSslKey);

namespace rust {
namespace cxxqtio1 {
QByteArray
qsslkeyToDer(const QSslKey& key)
{
  return key.toDer();
}

QByteArray
qsslkeyToPem(const QSslKey& key, const QByteArray& passPhrase)
{
  return key.toPem(passPhrase);
}
}

} // namespace rust
