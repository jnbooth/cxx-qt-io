#include "cxx-qt-io/qvariant.h"

#define CXX_QT_QVARIANT_CAN_CONVERT_IMPL(typeName, name)                       \
  bool qvariantCanConvert##name(const QVariant& variant)                       \
  {                                                                            \
    return variant.canConvert<typeName>();                                     \
  }

namespace rust {
namespace cxxqtlib1 {
namespace qvariant {

#ifdef CXX_QT_IO_REQUEST_FEATURE
CXX_QT_QVARIANT_CAN_CONVERT_IMPL(::QNetworkCookie, QNetworkCookie)
#endif

}
}
}
