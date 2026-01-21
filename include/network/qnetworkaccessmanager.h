#pragma once

#include <QtNetwork/QNetworkAccessManager>

namespace rust {
namespace cxxqtio1 {
using QNetworkAccessManagerOperation = QNetworkAccessManager::Operation;

QNetworkReply*
qnetworkaccessmanagerOptions(QNetworkAccessManager& manager,
                             const QNetworkRequest& request);

QNetworkReply*
qnetworkaccessmanagerPatch(QNetworkAccessManager& manager,
                           const QNetworkRequest& request,
                           const QByteArray& data);
QNetworkReply*
qnetworkaccessmanagerPatch(QNetworkAccessManager& manager,
                           const QNetworkRequest& request,
                           QIODevice* data);
QNetworkReply*
qnetworkaccessmanagerPatch(QNetworkAccessManager& manager,
                           const QNetworkRequest& request,
                           QHttpMultiPart* multiPart);

void
qnetworkaccessmanagerSetTransferTimeoutMsecs(QNetworkAccessManager& manaager,
                                             ::std::int64_t timeout);

::std::int64_t
qnetworkaccessmanagerTransferTimeoutMsecs(const QNetworkAccessManager& manager);

}
}
