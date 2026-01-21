#pragma once

#include <QtNetwork/QHstsPolicy>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<QHstsPolicy> : ::std::true_type
{};

namespace cxxqtio1 {
using QHstsPolicyPolicyFlag = QHstsPolicy::PolicyFlag;
using QHstsPolicyPolicyFlags = QHstsPolicy::PolicyFlags;

QHstsPolicy
qhstspolicyNew(const QDateTime& expiry,
               QHstsPolicyPolicyFlags flags,
               const QString& host);

QString
qhstspolicyHost(const QHstsPolicy& policy);

void
qhstspolicySetHost(QHstsPolicy& policy, const QString& host);

}

}
