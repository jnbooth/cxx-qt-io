#pragma once

#include <QtNetwork/QSsl>

using SslKeyType = QSsl::KeyType;
using SslEncodingFormat = QSsl::EncodingFormat;
using SslKeyAlgorithm = QSsl::KeyAlgorithm;
using SslAlternativeNameEntryType = QSsl::AlternativeNameEntryType;
using SslProtocol = QSsl::SslProtocol;
using SslOption = QSsl::SslOption;

#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
using SslAlertLevel = QSsl::AlertLevel;
using SslAlertType = QSsl::AlertType;
#endif

#if (QT_VERSION >= QT_VERSION_CHECK(6, 1, 0))
using SslImplementedClass = QSsl::ImplementedClass;
using SslSupportedFeature = QSsl::SupportedFeature;
#endif
