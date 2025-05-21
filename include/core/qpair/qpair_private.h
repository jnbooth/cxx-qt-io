#pragma once

#include "rust/cxx.h"
#include <QtCore/QPair>

namespace rust {

// This has static asserts in the cpp file to ensure this is valid.
template<typename T1, typename T2>
struct IsRelocatable<QPair<T1, T2>> : ::std::true_type
{};

}
