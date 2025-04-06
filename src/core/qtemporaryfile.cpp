#include "cxx-qt-io/qtemporaryfile.h"

namespace rust {
namespace cxxqtio1 {
std::unique_ptr<QTemporaryFile>
qtemporaryfileCreateNativeFile(QFile& file)
{
  return std::unique_ptr<QTemporaryFile>(
    QTemporaryFile::createNativeFile(file));
}

}
}
