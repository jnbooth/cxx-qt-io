#include "cxx-qt-io/qnetworkinterface.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QNetworkInterface, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QNetworkInterface>::value);
static_assert(
  !::std::is_trivially_copy_constructible<QNetworkInterface>::value);

static_assert(!::std::is_trivially_destructible<QNetworkInterface>::value);

static_assert(QTypeInfo<QNetworkInterface>::isRelocatable);
