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

void
qnetworkrequestSetTransferTimeoutMsecs(QNetworkRequest& request,
                                       ::std::int64_t timeout)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
  request.setTransferTimeout(::std::chrono::milliseconds{ timeout });
#else
  request.setTransferTimeout(int(timeout));
#endif
}

::std::int64_t
qnetworkrequestTransferTimeoutMsecs(const QNetworkRequest& request)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
  return request.transferTimeoutAsDuration().count();
#else
  return request.transferTimeout();
#endif
}

}
}
