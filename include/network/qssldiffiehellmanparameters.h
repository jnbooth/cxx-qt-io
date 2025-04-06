#pragma once

#include <QtNetwork/QSslDiffieHellmanParameters>

#include "rust/cxx.h"

using QSslDiffieHellmanParametersError = QSslDiffieHellmanParameters::Error;

namespace rust {
template<>
struct IsRelocatable<QSslDiffieHellmanParameters> : ::std::true_type
{};

namespace cxxqtio1 {
QSslDiffieHellmanParameters (*qssldiffiehellmanparametersDefaultParameters)() =
  QSslDiffieHellmanParameters::defaultParameters;

QSslDiffieHellmanParameters
qssldiffiehellmanparametersFromEncoded(QIODevice* device,
                                       QSsl::EncodingFormat encoding);

QSslDiffieHellmanParameters
qssldiffiehellmanparametersFromEncoded(const QByteArray& encoded,
                                       QSsl::EncodingFormat encoding);

}
}
