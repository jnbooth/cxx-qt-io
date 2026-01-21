#pragma once

#include <QtNetwork/QHostAddress>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QHostAddress> : ::std::true_type
{};

template<>
struct IsRelocatable<Q_IPV6ADDR> : ::std::true_type
{};

namespace cxxqtio1 {
using QHostAddressConversionModeFlag = QHostAddress::ConversionModeFlag;
using QHostAddressConversionMode = QHostAddress::ConversionMode;
using QHostAddressSpecialAddress = QHostAddress::SpecialAddress;

Q_IPV6ADDR
qhostaddressToIPv6Address(const QHostAddress& address);
}

}
