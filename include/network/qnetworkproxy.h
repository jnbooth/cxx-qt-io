#pragma once

#include <QtNetwork/QNetworkProxy>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QNetworkProxy> : ::std::true_type
{};

namespace cxxqtio1 {
using QNetworkProxyCapabilities = QNetworkProxy::Capabilities;
using QNetworkProxyCapability = QNetworkProxy::Capability;
using QNetworkProxyProxyType = QNetworkProxy::ProxyType;
}
}
