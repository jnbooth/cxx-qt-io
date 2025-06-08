#pragma once

#include <QtNetwork/QSslCipher>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslCipher> : ::std::true_type
{};

}
