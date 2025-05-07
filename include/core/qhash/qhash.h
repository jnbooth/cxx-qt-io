#pragma once

#include <cxx-qt-io/definitions.h>
#include <cxx-qt-lib/qhash.h>

#ifdef CXX_QT_IO_NETWORK_FEATURE
#include <QtNetwork/QNetworkRequest>
#endif

#include "qhash_i32_qvariant.h"

#ifdef CXX_QT_IO_NETWORK_FEATURE
using QHash_QNetworkRequestAttribute_QVariant =
  QHash<QNetworkRequest::Attribute, QVariant>;
#endif
