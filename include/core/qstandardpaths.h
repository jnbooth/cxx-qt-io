#pragma once

#include <QtCore/QStandardPaths>

using QStandardPathsLocateOption = QStandardPaths::LocateOption;
using QStandardPathsLocateOptions = QStandardPaths::LocateOptions;
using QStandardPathsStandardLocation = QStandardPaths::StandardLocation;

namespace rust {
namespace cxxqtio1 {
inline QString (*qstandardpathsDisplayName)(QStandardPaths::StandardLocation) =
  QStandardPaths::displayName;

inline QString (*qstandardpathsFindExecutable)(const QString& executableName,
                                               const QStringList& paths) =
  QStandardPaths::findExecutable;

inline QString (*qstandardpathsLocate)(QStandardPaths::StandardLocation type,
                                       const QString& fileName,
                                       QStandardPaths::LocateOptions) =
  QStandardPaths::locate;

inline QStringList (*qstandardpathsLocateAll)(
  QStandardPaths::StandardLocation type,
  const QString& fileName,
  QStandardPaths::LocateOptions) = QStandardPaths::locateAll;

inline QStringList (*qstandardpathsStandardLocations)(
  QStandardPaths::StandardLocation type) = QStandardPaths::standardLocations;

inline QString (*qstandardpathsWritableLocation)(
  QStandardPaths::StandardLocation type) = QStandardPaths::writableLocation;

}
}
