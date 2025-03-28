#pragma once

#include <QtCore/QByteArray>

#include "qlist_private.h"

using QHttpHeadersEntry = QList<::std::pair<QByteArray, QByteArray>>;
using QList_QHttpHeadersEntry = QList<QHttpHeadersEntry>;
