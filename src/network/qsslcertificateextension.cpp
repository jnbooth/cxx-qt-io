#include "cxx-qt-io/qsslcertificateextension.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QSslCertificateExtension, { ::std::size_t a0; });

static_assert(
  !::std::is_trivially_copy_assignable<QSslCertificateExtension>::value);
static_assert(
  !::std::is_trivially_copy_constructible<QSslCertificateExtension>::value);

static_assert(
  !::std::is_trivially_destructible<QSslCertificateExtension>::value);

static_assert(QTypeInfo<QSslCertificateExtension>::isRelocatable);
