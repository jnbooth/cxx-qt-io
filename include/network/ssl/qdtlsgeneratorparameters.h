#pragma once

#include <QtNetwork/QDtls>

#include "rust/cxx.h"

using QDtlsGeneratorParameters = QDtls::GeneratorParameters;

namespace rust {
template<>
struct IsRelocatable<QDtlsGeneratorParameters> : ::std::true_type
{};

}
