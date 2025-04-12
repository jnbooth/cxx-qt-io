#pragma once

#include <QtNetwork/QSslPreSharedKeyAuthenticator>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslPreSharedKeyAuthenticator> : ::std::true_type
{};

}
