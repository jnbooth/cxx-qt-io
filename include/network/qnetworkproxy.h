#pragma once

#include <QtNetwork/QNetworkProxy>

#include "rust/cxx.h"

using QNetworkProxyCapabilities = QNetworkProxy::Capabilities;
using QNetworkProxyCapability = QNetworkProxy::Capability;
using QNetworkProxyProxyType = QNetworkProxy::ProxyType;

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
