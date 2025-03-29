#pragma once

#include <QtCore/QIODevice>

using OpenModeFlag = QIODeviceBase::OpenModeFlag;
using OpenMode = QIODeviceBase::OpenMode;

namespace rust {
namespace cxxqtio1 {
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

}
}
