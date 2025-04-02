#pragma once

#include <QtNetwork/QNetworkAddressEntry>

#include "rust/cxx.h"

using QNetworkAddressEntryDnsEligibilityStatus = QNetworkAddressEntry::DnsEligibilityStatus;

namespace rust {
template<>
struct IsRelocatable<QNetworkAddressEntry> : ::std::true_type
{};

} // namespace rust
