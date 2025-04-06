#pragma once

#include <QtNetwork/QNetworkRequest>

#include "rust/cxx.h"

using QNetworkRequestAttribute = QNetworkRequest::Attribute;
using QNetworkRequestCacheLoadControl = QNetworkRequest::CacheLoadControl;
using QNetworkRequestKnownHeaders = QNetworkRequest::KnownHeaders;
using QNetworkRequestLoadControl = QNetworkRequest::LoadControl;
using QNetworkRequestPriority = QNetworkRequest::Priority;
using QNetworkRequestRedirectPolicy = QNetworkRequest::RedirectPolicy;

namespace rust {
template<>
struct IsRelocatable<QNetworkRequest> : ::std::true_type
{};

namespace cxxqtio1 {
QVariant
qnetworkrequestAttribute(const QNetworkRequest& request,
                         QNetworkRequest::Attribute code);

}
}
