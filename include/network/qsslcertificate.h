#pragma once

#include <QtNetwork/QSslCertificate>

#include "cxx-qt-io/qmultimap.h"
#include "rust/cxx.h"

namespace rust {

template<>
struct IsRelocatable<QSslCertificate> : ::std::true_type
{};

namespace cxxqtio1 {
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
}

template<>
struct IsRelocatable<cxxqtio1::SubjectAlternativeNamesMap> : ::std::true_type
{};

template<>
struct IsRelocatable<cxxqtio1::SubjectAlternativeNamesIter> : ::std::true_type
{};

template<>
struct IsRelocatable<cxxqtio1::SubjectAlternativeNamesValues> : ::std::true_type
{};
}
