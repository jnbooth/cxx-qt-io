#pragma once

#include <QtNetwork/QNetworkInterface>

#include "rust/cxx.h"

using QNetworkInterfaceInterfaceFlag = QNetworkInterface::InterfaceFlag;
using QNetworkInterfaceInterfaceFlags = QNetworkInterface::InterfaceFlags;
using QNetworkInterfaceInterfaceType = QNetworkInterface::InterfaceType;

namespace rust {
template<>
struct IsRelocatable<QNetworkInterface> : ::std::true_type
{};

namespace cxxqtio1 {
inline QList<QHostAddress> (*qnetworkinterfaceAllAddresses)() =
  QNetworkInterface::allAddresses;

inline QList<QNetworkInterface> (*qnetworkinterfaceAllInterfaces)() =
  QNetworkInterface::allInterfaces;

inline QNetworkInterface (*qnetworkinterfaceInterfaceFromIndex)(int) =
  QNetworkInterface::interfaceFromIndex;

inline QNetworkInterface (*qnetworkinterfaceInterfaceFromName)(const QString&) =
  QNetworkInterface::interfaceFromName;

inline int (*qnetworkinterfaceInterfaceIndexFromName)(const QString&) =
  QNetworkInterface::interfaceIndexFromName;

inline QString (*qnetworkinterfaceInterfaceNameFromIndex)(int) =
  QNetworkInterface::interfaceNameFromIndex;

}

}
