#pragma once

#include <QtNetwork/QHostAddress>

#include "rust/cxx.h"

using NetworkLayerProtocol = QAbstractSocket::NetworkLayerProtocol;
using AddressConversionModeFlag = QHostAddress::ConversionModeFlag;
using AddressConversionMode = QHostAddress::ConversionMode;
using SpecialHostAddress = QHostAddress::SpecialAddress;

namespace rust {
template<>
struct IsRelocatable<QHostAddress> : ::std::true_type
{};

template<>
struct IsRelocatable<Q_IPV6ADDR> : ::std::true_type
{};

namespace cxxqtio1 {
Q_IPV6ADDR
qhostaddressToIPv6Address(const QHostAddress& address);

inline ::std::pair<QHostAddress, i32> (*qhostaddressParseSubnet)(
  const QString&) = QHostAddress::parseSubnet;
}

} // namespace rust
