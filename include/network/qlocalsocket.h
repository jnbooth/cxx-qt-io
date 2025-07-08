#pragma once

#include <QtNetwork/QLocalSocket>

using QLocalSocketLocalSocketError = QLocalSocket::LocalSocketError;
using QLocalSocketLocalSocketState = QLocalSocket::LocalSocketState;

#if (QT_VERSION >= QT_VERSION_CHECK(6, 2, 0))
using QLocalSocketSocketOption = QLocalSocket::SocketOption;
using QLocalSocketSocketOptions = QLocalSocket::SocketOptions;
#else
enum QLocalSocketSocketOption
{
  NoOptions = 0x00,
  AbstractNamespaceOption = 0x01
};
using QLocalSocketSocketOptions = QFlags<QLocalSocketSocketOption>;
#endif
