#pragma once

#include <QtNetwork/QNetworkProxy>

#include "rust/cxx.h"

using ProxyCapabilities = QNetworkProxy::Capabilities;
using ProxyCapability = QNetworkProxy::Capability;
using ProxyType = QNetworkProxy::ProxyType;

namespace rust {
template<>
struct IsRelocatable<QNetworkProxy> : ::std::true_type
{};

namespace cxxqtio1 {
inline QNetworkProxy (*qnetworkproxyApplicationProxy)() =
  QNetworkProxy::applicationProxy;

inline void (*qnetworkproxySetApplicationProxy)(const QNetworkProxy&) =
  QNetworkProxy::setApplicationProxy;
}

} // namespace rust
