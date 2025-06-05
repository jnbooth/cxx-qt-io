#include "cxx-qt-io/qnetworkaccessmanager.h"

static const QByteArray verbOptions = "OPTION";
static const QByteArray verbPatch = "PATCH";

namespace rust {
namespace cxxqtio1 {
QNetworkReply*
qnetworkaccessmanagerOptions(QNetworkAccessManager& manager,
                             const QNetworkRequest& request)
{
  return manager.sendCustomRequest(request, verbOptions);
}

QNetworkReply*
qnetworkaccessmanagerPatch(QNetworkAccessManager& manager,
                           const QNetworkRequest& request,
                           const QByteArray& data)
{
  return manager.sendCustomRequest(request, verbPatch, data);
}
QNetworkReply*
qnetworkaccessmanagerPatch(QNetworkAccessManager& manager,
                           const QNetworkRequest& request,
                           QIODevice* data)
{

  return manager.sendCustomRequest(request, verbPatch, data);
}
QNetworkReply*
qnetworkaccessmanagerPatch(QNetworkAccessManager& manager,
                           const QNetworkRequest& request,
                           QHttpMultiPart* multiPart)
{
  return manager.sendCustomRequest(request, verbPatch, multiPart);
}

void
qnetworkaccessmanagerSetTransferTimeoutMsecs(QNetworkAccessManager& manager,
                                             ::std::int64_t timeout)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
  manager.setTransferTimeout(::std::chrono::milliseconds{ timeout });
#else
  manager.setTransferTimeout(int(timeout));
#endif
}

::std::int64_t
qnetworkaccessmanagerTransferTimeoutMsecs(const QNetworkAccessManager& manager)
{
#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
  return manager.transferTimeoutAsDuration().count();
#else
  return manager.transferTimeout();
#endif
}

}
}
