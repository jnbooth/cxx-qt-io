#pragma once

#include <QtNetwork/QSslDiffieHellmanParameters>

#include "rust/cxx.h"

using QSslDiffieHellmanParametersError = QSslDiffieHellmanParameters::Error;

namespace rust {
template<>
struct IsRelocatable<QSslDiffieHellmanParameters> : ::std::true_type
{};

namespace cxxqtio1 {
inline QSslDiffieHellmanParameters (
  *qssldiffiehellmanparametersDefaultParameters)() =
  QSslDiffieHellmanParameters::defaultParameters;

QSslDiffieHellmanParameters
qssldiffiehellmanparametersFromEncoded(QIODevice* device,
                                       QSsl::EncodingFormat encoding);

QSslDiffieHellmanParameters
qssldiffiehellmanparametersFromEncoded(const QByteArray& encoded,
                                       QSsl::EncodingFormat encoding);

QString
qssldiffiehellmanparametersDebug(const QSslDiffieHellmanParameters& params);

}
}
