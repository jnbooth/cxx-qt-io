#pragma once

#include <QtNetwork/QSslConfiguration>

#include "rust/cxx.h"

using QSslConfigurationNextProtocolNegotiationStatus =
  QSslConfiguration::NextProtocolNegotiationStatus;

namespace rust {
template<>
struct IsRelocatable<QSslConfiguration> : ::std::true_type
{};

namespace cxxqtio1 {
::rust::String
qsslconfigurationALPNProtocolHTTP2();

::rust::String
qsslconfigurationNextProtocolHttp1_1();

}
}
