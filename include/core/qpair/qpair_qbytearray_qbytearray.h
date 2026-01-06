//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qpair/generate.sh
#pragma once
#include "qpair_private.h"
#include <QtCore/QByteArray>

using QPair_QByteArray_QByteArray = ::std::pair<::QByteArray, ::QByteArray>;

namespace rust {

// This has static asserts in the cpp file to ensure this is valid.
template<>
struct IsRelocatable<QPair<::QByteArray, ::QByteArray>> : ::std::true_type
{};

}
