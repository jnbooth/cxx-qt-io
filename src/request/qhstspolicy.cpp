#include "cxx-qt-io/qhstspolicy.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QHstsPolicy);

namespace rust {
namespace cxxqtio1 {
QHstsPolicy
qhstspolicyNew(const QDateTime& expiry,
               QHstsPolicyPolicyFlags flags,
               const QString& host)
{
  return QHstsPolicy(expiry, flags, host);
}

QString
qhstspolicyHost(const QHstsPolicy& policy)
{
  return policy.host();
}

void
qhstspolicySetHost(QHstsPolicy& policy, const QString& host)
{
  policy.setHost(host);
}

}
}
