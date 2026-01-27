#include "cxx-qt-io/qhostaddress.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QHostAddress);

assert_plain_struct(Q_IPV6ADDR, { ::std::uint8_t c[16]; });
