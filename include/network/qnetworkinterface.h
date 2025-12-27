#pragma once

#include <QtNetwork/QNetworkInterface>

#include "rust/cxx.h"

using QNetworkInterfaceInterfaceFlag = QNetworkInterface::InterfaceFlag;
using QNetworkInterfaceInterfaceFlags = QNetworkInterface::InterfaceFlags;
using QNetworkInterfaceInterfaceType = QNetworkInterface::InterfaceType;

namespace rust {
template<>
struct IsRelocatable<QNetworkInterface> : ::std::true_type
{};
}
