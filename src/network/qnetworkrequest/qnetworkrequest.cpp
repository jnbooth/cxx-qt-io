#include "cxx-qt-io/qnetworkrequest.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QNetworkRequest);

namespace rust {
namespace cxxqtio1 {
QVariant
qnetworkrequestAttribute(const QNetworkRequest& request,
                         QNetworkRequest::Attribute code)
{
  return request.attribute(code);
}

}
}
