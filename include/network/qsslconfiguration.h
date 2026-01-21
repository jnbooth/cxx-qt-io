#pragma once

#include <QtNetwork/QSslConfiguration>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslConfiguration> : ::std::true_type
{};

namespace cxxqtio1 {
using QSslConfigurationNextProtocolNegotiationStatus =
  QSslConfiguration::NextProtocolNegotiationStatus;

::rust::String
qsslconfigurationALPNProtocolHTTP2();

::rust::String
qsslconfigurationNextProtocolHttp1_1();

}
}
