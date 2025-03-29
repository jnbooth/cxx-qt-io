#pragma once

#include <QtCore/QTemporaryFile>

namespace rust {
namespace cxxqtio1 {
inline QTemporaryFile* (*qtemporaryfileCreateNativeFile)(QFile&) =
  QTemporaryFile::createNativeFile;

}
} // namespace rust
