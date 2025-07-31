#include "cxx-qt-io/qssldiffiehellmanparameters.h"

#include <QtCore/QDebug>
#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QSslDiffieHellmanParameters);

namespace rust {
namespace cxxqtio1 {
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

const int startAt = sizeof("QSslDiffieHellmanParameters(");

QString
qssldiffiehellmanparametersDebug(const QSslDiffieHellmanParameters& params)
{
  QString res;
  QDebug serializer{ &res };
  serializer << params;
  return res.sliced(startAt, res.length() - 3 - startAt);
}

}
}
