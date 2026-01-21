#pragma once

#include <QtNetwork/QSslDiffieHellmanParameters>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QSslDiffieHellmanParameters> : ::std::true_type
{};

namespace cxxqtio1 {
using QSslDiffieHellmanParametersError = QSslDiffieHellmanParameters::Error;

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
