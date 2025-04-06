#pragma once

#include <QtCore/QTemporaryFile>

namespace rust {
namespace cxxqtio1 {
std::unique_ptr<QTemporaryFile>
qtemporaryfileCreateNativeFile(QFile& file);

}
}
