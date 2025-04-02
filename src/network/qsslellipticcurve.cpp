#include "cxx-qt-io/qsslellipticcurve.h"

#include <cxx-qt-io/assertion_utils.h>

assert_alignment_and_size(QSslEllipticCurve, { ::std::int32_t id; });

static_assert(::std::is_trivially_copy_assignable<QSslEllipticCurve>::value);
static_assert(::std::is_trivially_copy_constructible<QSslEllipticCurve>::value);

static_assert(::std::is_trivially_destructible<QSslEllipticCurve>::value);

static_assert(QTypeInfo<QSslEllipticCurve>::isRelocatable);
