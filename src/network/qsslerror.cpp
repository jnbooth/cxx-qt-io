#include "cxx-qt-io/qsslerror.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QSslError, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QSslError>::value);
static_assert(!::std::is_trivially_copy_constructible<QSslError>::value);

static_assert(!::std::is_trivially_destructible<QSslError>::value);

static_assert(QTypeInfo<QSslError>::isRelocatable);
