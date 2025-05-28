#pragma once

#include <QtCore/QObject>

namespace rust {
namespace cxxqtio1 {
inline void
qobjectDelete(QObject* object)
{
  delete object;
}

}
}
