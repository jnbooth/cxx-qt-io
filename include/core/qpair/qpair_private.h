#pragma once

#include "rust/cxx.h"

namespace rust {

// This has static asserts in the cpp file to ensure this is valid.
template<typename K, typename V>
struct IsRelocatable<::std::pair<K, V>> : ::std::true_type
{};

}
