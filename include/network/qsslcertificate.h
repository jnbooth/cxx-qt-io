#pragma once

#include <QtNetwork/QSslCertificate>

#include "rust/cxx.h"

using QSslCertificatePatternSyntax = QSslCertificate::PatternSyntax;
using QSslCertificateSubjectInfo = QSslCertificate::SubjectInfo;

namespace rust {
template<>
struct IsRelocatable<QSslCertificate> : ::std::true_type
{};

namespace cxxqtio1 {
QList<QSslCertificate> (*qsslcertificateFromData)(const QByteArray&,
                                                  QSsl::EncodingFormat) =
  QSslCertificate::fromData;

QList<QSslCertificate> (*qsslcertificateFromDevice)(QIODevice*,
                                                    QSsl::EncodingFormat) =
  QSslCertificate::fromDevice;

QList<QSslCertificate> (*qsslcertificateFromPath)(const QString&,
                                                  QSsl::EncodingFormat,
                                                  QSslCertificatePatternSyntax) =
  QSslCertificate::fromPath;

bool (*qsslcertificateImportPkcs12)(QIODevice* device,
                                    QSslKey* key,
                                    QSslCertificate* certificate,
                                    QList<QSslCertificate>* caCertificates,
                                    const QByteArray& passPhrase) =
  QSslCertificate::importPkcs12;

QList<QSslError> (*qsslcertificateVerify)(const QList<QSslCertificate>&,
                                          const QString&) =
  QSslCertificate::verify;

}

} // namespace rust
