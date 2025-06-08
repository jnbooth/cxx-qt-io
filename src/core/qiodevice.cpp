#include "cxx-qt-io/qiodevice.h"

#include <QtCore/QFile>

using OpenModeFlag = QIODeviceBase::OpenModeFlag;
using OpenMode = QIODeviceBase::OpenMode;

namespace rust {
namespace cxxqtio1 {
bool
qiodeviceIsOpen(const QIODevice& device)
{
  return device.isOpen();
}

bool
qiodeviceIsReadable(const QIODevice& device)
{
  return device.isReadable();
}

bool
qiodeviceIsSequential(const QIODevice& device)
{
  return device.isSequential();
}

bool
qiodeviceIsTextModeEnabled(const QIODevice& device)
{
  return device.isTextModeEnabled();
}

bool
qiodeviceIsTransactionStarted(const QIODevice& device)
{
  return device.isTransactionStarted();
}

bool
qiodeviceIsWritable(const QIODevice& device)
{
  return device.isWritable();
}

const QIODeviceOpenMode qFileOnlyFlags =
  QIODevice::NewOnly | QIODevice::ExistingOnly;

bool
qiodeviceOpen(QIODevice& device, QIODeviceOpenMode mode)
{
  if ((mode & qFileOnlyFlags) &&
      !device.metaObject()->inherits(&QFile::staticMetaObject)) {
    return device.open(mode & ~qFileOnlyFlags);
  }
  return device.open(mode);
}

}
}
