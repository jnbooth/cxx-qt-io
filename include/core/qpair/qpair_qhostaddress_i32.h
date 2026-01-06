//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qpair/generate.sh
#pragma once
#include "qpair_private.h"
#include <QtNetwork/QHostAddress>

using QPair_QHostAddress_i32 = ::std::pair<::QHostAddress, ::std::int32_t>;

namespace rust {

// This has static asserts in the cpp file to ensure this is valid.
template<>
struct IsRelocatable<QPair<::QHostAddress, ::std::int32_t>> : ::std::true_type
{};

}
