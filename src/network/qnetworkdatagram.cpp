#include "cxx-qt-io/qnetworkdatagram.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QNetworkDatagram, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QNetworkDatagram>::value);
static_assert(!::std::is_trivially_copy_constructible<QNetworkDatagram>::value);

static_assert(!::std::is_trivially_destructible<QNetworkDatagram>::value);

static_assert(QTypeInfo<QNetworkDatagram>::isRelocatable);

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
