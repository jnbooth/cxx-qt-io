#include "cxx-qt-io/qssldiffiehellmanparameters.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QSslDiffieHellmanParameters);

namespace rust {
namespace cxxqtio1 {
QSslDiffieHellmanParameters (*defaultParameters)() =
  QSslDiffieHellmanParameters::defaultParameters;

QSslDiffieHellmanParameters
qssldiffiehellmanparametersFromEncoded(QIODevice* device,
                                       QSsl::EncodingFormat encoding)
{
  return QSslDiffieHellmanParameters::fromEncoded(device, encoding);
}

QSslDiffieHellmanParameters
qssldiffiehellmanparametersFromEncoded(const QByteArray& encoded,
                                       QSsl::EncodingFormat encoding)
{
  return QSslDiffieHellmanParameters::fromEncoded(encoded, encoding);
}

}
}
