#pragma once

#include <QtNetwork/QNetworkAddressEntry>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QNetworkAddressEntry> : ::std::true_type
{};

namespace cxxqtio1 {
using QNetworkAddressEntryDnsEligibilityStatus =
  QNetworkAddressEntry::DnsEligibilityStatus;
}
}
