#pragma once

#include <QtNetwork/QSslError>

#include "rust/cxx.h"

using SslError = QSslError::SslError;

namespace rust {
template<>
struct IsRelocatable<QSslError> : ::std::true_type
{};

} // namespace rust
