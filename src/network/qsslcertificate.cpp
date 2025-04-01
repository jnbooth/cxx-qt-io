#include "cxx-qt-io/qsslcertificate.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QSslCertificate, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QSslCertificate>::value);
static_assert(!::std::is_trivially_copy_constructible<QSslCertificate>::value);

static_assert(!::std::is_trivially_destructible<QSslCertificate>::value);

static_assert(QTypeInfo<QSslCertificate>::isRelocatable);
