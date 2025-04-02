#pragma once

#include <QtNetwork/QSsl>

using QSslKeyType = QSsl::KeyType;
using QSslEncodingFormat = QSsl::EncodingFormat;
using QSslKeyAlgorithm = QSsl::KeyAlgorithm;
using QSslAlternativeNameEntryType = QSsl::AlternativeNameEntryType;
using QSslSslProtocol = QSsl::SslProtocol;
using QSslSslOption = QSsl::SslOption;

#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
using QSslAlertLevel = QSsl::AlertLevel;
using QSslAlertType = QSsl::AlertType;
#endif

#if (QT_VERSION >= QT_VERSION_CHECK(6, 1, 0))
using QSslImplementedClass = QSsl::ImplementedClass;
using QSslSupportedFeature = QSsl::SupportedFeature;
#endif
