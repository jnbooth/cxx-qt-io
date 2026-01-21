#include "cxx-qt-io/qiodevice.h"

#include <QtCore/QIODevice>

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

bool
qiodeviceOpen(QIODevice& device, QIODeviceOpenMode mode)
{
  return device.open(mode);
}
}
}
