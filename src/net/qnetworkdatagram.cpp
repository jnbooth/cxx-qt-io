#include "cxx-qt-io/qnetworkdatagram.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QNetworkDatagram);

namespace rust {
namespace cxxqtio1 {

QNetworkDatagram
qnetworkdatagramMakeReply(const QNetworkDatagram& datagram,
                          const QByteArray& payload)
{
  return datagram.makeReply(payload);
}
}
}
