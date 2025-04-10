#pragma once

#include <QtCore/QDir>

namespace rust {
namespace cxxqtio1 {
inline void (*qdirAddSearchPath)(const QString&,
                                 const QString&) = QDir::addSearchPath;
inline QString (*qdirCleanPath)(const QString&) = QDir::cleanPath;
inline QString (*qdirCurrentPath)() = QDir::currentPath;
inline QString (*qdirHomePath)() = QDir::homePath;
inline QString (*qdirRootPath)() = QDir::rootPath;
inline QStringList (*qdirSearchPaths)(const QString&) = QDir::searchPaths;
inline bool (*qdirSetCurrent)(const QString&) = QDir::setCurrent;
inline void (*qdirSetSearchPaths)(const QString&,
                                  const QStringList&) = QDir::setSearchPaths;
inline QString (*qdirTempPath)() = QDir::tempPath;
}
}
