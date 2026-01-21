#pragma once

#include <QtCore/QIODevice>

namespace rust {
namespace cxxqtio1 {
using QIODeviceOpenModeFlag = QIODeviceBase::OpenModeFlag;
using QIODeviceOpenMode = QIODeviceBase::OpenMode;

bool
qiodeviceIsOpen(const QIODevice& device);

bool
qiodeviceIsReadable(const QIODevice& device);

bool
qiodeviceIsSequential(const QIODevice& device);

bool
qiodeviceIsTextModeEnabled(const QIODevice& device);

bool
qiodeviceIsTransactionStarted(const QIODevice& device);

bool
qiodeviceIsWritable(const QIODevice& device);

bool
qiodeviceOpen(QIODevice& device, QIODeviceOpenMode mode);

}
}
