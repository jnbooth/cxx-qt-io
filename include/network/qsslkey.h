#pragma once

#include <QtNetwork/QSslKey>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslKey> : ::std::true_type
{};

namespace cxxqtio1 {
QByteArray
qsslkeyToDer(const QSslKey& key);

QByteArray
qsslkeyToPem(const QSslKey& key, const QByteArray &passPhrase);
}

} // namespace rust
