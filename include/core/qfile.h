#pragma once

#include <QtCore/QFile>

namespace rust {
namespace cxxqtio1 {
inline QString (*qfileDecodeName)(const QByteArray&) = QFile::decodeName;
inline QByteArray (*qfileEncodeName)(const QString&) = QFile::encodeName;

#if (QT_VERSION >= QT_VERSION_CHECK(6, 9, 0))
inline bool (*qfileSupportsMoveToTrash)() = QFile::supportsMoveToTrash;
#endif
}
}
