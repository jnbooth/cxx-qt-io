#pragma once

#include <QtCore/QTemporaryFile>

namespace rust {
namespace cxxqtio1 {
QTemporaryFile* (*qtemporaryfileCreateNativeFile)(QFile&) =
  QTemporaryFile::createNativeFile;

}
}
