#pragma once

#include <QtNetwork/QSslCertificate>

#include "cxx-qt-io/qmultimap.h"
#include "rust/cxx.h"

using QSslCertificatePatternSyntax = QSslCertificate::PatternSyntax;
using QSslCertificateSubjectInfo = QSslCertificate::SubjectInfo;
using QSslAlternativeNameEntryType = QSsl::AlternativeNameEntryType;
using SubjectAlternativeNamesMap =
  QMultiMap<QSsl::AlternativeNameEntryType, QString>;
using SubjectAlternativeNamesIter =
  ::rust::cxxqtio1::QMultiMapEntryIter<QSsl::AlternativeNameEntryType, QString>;
using SubjectAlternativeNamesKeys =
  ::rust::cxxqtio1::QMultiMapKeyIter<QSsl::AlternativeNameEntryType, QString>;
using SubjectAlternativeNamesValues =
  ::rust::cxxqtio1::QMultiMapValueIter<QSsl::AlternativeNameEntryType, QString>;

namespace rust {
template<>
struct IsRelocatable<QSslCertificate> : ::std::true_type
{};

template<>
struct IsRelocatable<SubjectAlternativeNamesMap> : ::std::true_type
{};

template<>
struct IsRelocatable<SubjectAlternativeNamesIter> : ::std::true_type
{};

template<>
struct IsRelocatable<SubjectAlternativeNamesValues> : ::std::true_type
{};

namespace cxxqtio1 {
inline QList<QSslCertificate> (*qsslcertificateFromData)(const QByteArray&,
                                                         QSsl::EncodingFormat) =
  QSslCertificate::fromData;

inline QList<QSslCertificate> (*qsslcertificateFromDevice)(
  QIODevice*,
  QSsl::EncodingFormat) = QSslCertificate::fromDevice;

inline QList<QSslCertificate> (*qsslcertificateFromPath)(
  const QString&,
  QSsl::EncodingFormat,
  QSslCertificatePatternSyntax) = QSslCertificate::fromPath;

inline bool (*qsslcertificateImportPkcs12)(
  QIODevice* device,
  QSslKey* key,
  QSslCertificate* certificate,
  QList<QSslCertificate>* caCertificates,
  const QByteArray& passPhrase) = QSslCertificate::importPkcs12;

inline QList<QSslError> (*qsslcertificateVerify)(const QList<QSslCertificate>&,
                                                 const QString&) =
  QSslCertificate::verify;
}

}
