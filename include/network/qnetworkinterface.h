#pragma once

#include <QtNetwork/QNetworkInterface>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QNetworkInterface> : ::std::true_type
{};

namespace cxxqtio1 {
using QNetworkInterfaceInterfaceFlag = QNetworkInterface::InterfaceFlag;
using QNetworkInterfaceInterfaceFlags = QNetworkInterface::InterfaceFlags;
using QNetworkInterfaceInterfaceType = QNetworkInterface::InterfaceType;
}
}
