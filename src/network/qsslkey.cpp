#include "cxx-qt-io/qsslkey.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QSslKey, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QSslKey>::value);
static_assert(!::std::is_trivially_copy_constructible<QSslKey>::value);

static_assert(!::std::is_trivially_destructible<QSslKey>::value);

static_assert(QTypeInfo<QSslKey>::isRelocatable);

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
