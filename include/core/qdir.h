#pragma once

#include <QtCore/QDir>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QDir> : ::std::true_type
{};
}
