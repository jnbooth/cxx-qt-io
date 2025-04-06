#pragma once

#include <QtNetwork/QNetworkDatagram>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QNetworkDatagram> : ::std::true_type
{};

namespace cxxqtio1 {
QNetworkDatagram
qnetworkdatagramMakeReply(const QNetworkDatagram& datagram,
                          const QByteArray& payload);
}

}
