#include "cxx-qt-io/qhostaddress.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QHostAddress);

assert_alignment_and_size(Q_IPV6ADDR, { ::std::uint8_t c[16]; });

static_assert(::std::is_trivially_copy_assignable<Q_IPV6ADDR>::value);
static_assert(::std::is_trivially_copy_constructible<Q_IPV6ADDR>::value);

static_assert(::std::is_trivially_destructible<Q_IPV6ADDR>::value);

static_assert(QTypeInfo<Q_IPV6ADDR>::isRelocatable);

namespace rust {
namespace cxxqtio1 {

Q_IPV6ADDR
qhostaddressToIPv6Address(const QHostAddress& address)
{
  return address.toIPv6Address();
}
}

}
