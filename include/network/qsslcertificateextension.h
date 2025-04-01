#pragma once

#include <QtNetwork/QSslCertificateExtension>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslCertificateExtension> : ::std::true_type
{};

} // namespace rust
