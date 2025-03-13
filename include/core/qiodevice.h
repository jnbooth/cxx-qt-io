#pragma once

#include <QtCore/QIODevice>

using QIODeviceBase_OpenModeFlag = QIODeviceBase::OpenModeFlag;

using QIODeviceBase_OpenMode = QIODeviceBase::OpenMode;

namespace rust {
namespace cxxqtio1 {
inline bool
qiodeviceIsOpen(const QIODevice& device)
{
  return device.isOpen();
}

inline bool
qiodeviceIsReadable(const QIODevice& device)
{
  return device.isReadable();
}

inline bool
qiodeviceIsSequential(const QIODevice& device)
{
  return device.isSequential();
}

inline bool
qiodeviceIsTextModeEnabled(const QIODevice& device)
{
  return device.isTextModeEnabled();
}

inline bool
qiodeviceIsTransactionStarted(const QIODevice& device)
{
  return device.isTransactionStarted();
}

inline bool
qiodeviceIsWritable(const QIODevice& device)
{
  return device.isWritable();
}

}
}
