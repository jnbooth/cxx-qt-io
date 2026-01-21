#pragma once

#include <QtNetwork/QSslError>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslError> : ::std::true_type
{};

namespace cxxqtio1 {
using QSslErrorSslError = QSslError::SslError;
}
}
